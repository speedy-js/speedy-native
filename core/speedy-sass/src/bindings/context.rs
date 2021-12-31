use crate::bindings::options::SassOptions;
use crate::bindings::ptr::Unique;
use crate::bindings::util;
use crate::options::{Options, OutputStyle};
use crate::sass_clib::root::{
  sass_compile_data_context, sass_compile_file_context, sass_context_get_error_message,
  sass_context_get_error_status, sass_context_get_options, sass_context_get_output_string,
  sass_data_context_get_context, sass_delete_data_context, sass_delete_file_context,
  sass_file_context_get_context, sass_make_data_context, sass_make_file_context, Sass_Context,
  Sass_Data_Context, Sass_File_Context,
};
use libc::strdup;
use std::ffi;
use std::path::Path;
use std::sync::{Arc, RwLock};

pub struct SassContext {
  // Need Unique to send between threads, libsass is thread safe
  pub raw: Unique<Sass_Context>,
  pub options: Arc<RwLock<SassOptions>>,
}

pub struct SassFileContext {
  // Need Unique to send between threads, libsass is thread safe
  context: Unique<Sass_File_Context>,
  pub sass_context: SassContext,
}

pub struct SassDataContext {
  // Need Unique to send between threads, libsass is thread safe
  context: Unique<Sass_Data_Context>,
  pub sass_context: SassContext,
}

pub enum Context {
  File(SassFileContext),
  Data(SassDataContext),
}

impl Context {
  fn make_sass_context(c_sass_context: *mut Sass_Context) -> SassContext {
    let options = unsafe { sass_context_get_options(c_sass_context) };
    let sass_options = Arc::new(RwLock::new(SassOptions {
      raw: unsafe { Unique::new(options) },
    }));

    SassContext {
      raw: unsafe { Unique::new(c_sass_context) },
      options: sass_options,
    }
  }

  pub fn new_data(data: &str) -> Context {
    let c_str = ffi::CString::new(data).unwrap();
    let data_context = unsafe { sass_make_data_context(strdup(c_str.as_ptr())) };
    let data_sass_context = unsafe { sass_data_context_get_context(data_context) };
    let sass_context = Context::make_sass_context(data_sass_context);

    Context::Data(SassDataContext {
      context: unsafe { Unique::new(data_context) },
      sass_context,
    })
  }

  pub fn new_file<P: AsRef<Path>>(path: P) -> Result<Context, String> {
    let c_str = ffi::CString::new(
      path
        .as_ref()
        .to_str()
        .ok_or_else(|| "str conversation failed".to_string())?,
    )
    .map_err(|e| format!("Failed to create CString: {}", e))?;
    let file_context = unsafe { sass_make_file_context(c_str.as_ptr()) };
    let file_sass_context = unsafe { sass_file_context_get_context(file_context) };
    let sass_context = Context::make_sass_context(file_sass_context);

    Ok(Context::File(SassFileContext {
      context: unsafe { Unique::new(file_context) },
      sass_context,
    }))
  }

  pub fn set_options(&mut self, options: Options) {
    let mut sass_options = match *self {
      Context::File(ref mut s) => (*s.sass_context.options).write().unwrap(),
      Context::Data(ref mut s) => (*s.sass_context.options).write().unwrap(),
    };
    sass_options.set_output_style(options.output_style);
    sass_options.set_precision(options.precision);
    if options.indented_syntax {
      sass_options.set_is_indented_syntax();
    }
    if !options.include_paths.is_empty() {
      sass_options.set_include_path(options.include_paths);
    }
  }

  pub fn compile(&mut self) -> Result<String, String> {
    let ctx_out = match *self {
      Context::File(ref mut s) => unsafe {
        sass_compile_file_context(s.context.get_mut());
        s.sass_context.raw.get_mut()
      },
      Context::Data(ref mut s) => unsafe {
        sass_compile_data_context(s.context.get_mut());
        s.sass_context.raw.get_mut()
      },
    };

    let error_status = unsafe { sass_context_get_error_status(ctx_out) };
    let error_message = unsafe { sass_context_get_error_message(ctx_out) };
    let output_string = unsafe { sass_context_get_output_string(ctx_out) };

    if error_status != 0 {
      if !error_message.is_null() {
        Result::Err(util::to_string(error_message))
      } else {
        Result::Err("An error occurred; no error message available.".to_string())
      }
    } else {
      Result::Ok(util::to_string(output_string))
    }
  }

  pub fn set_output_style(&mut self, output_style: OutputStyle) {
    match *self {
      Context::File(ref mut s) => {
        let mut options = (*s.sass_context.options).write().unwrap();
        options.set_output_style(output_style);
      }
      Context::Data(ref mut s) => {
        let mut options = (*s.sass_context.options).write().unwrap();
        options.set_output_style(output_style);
      }
    };
  }
}

impl Drop for Context {
  fn drop(&mut self) {
    match *self {
      Context::File(ref mut s) => unsafe { sass_delete_file_context(s.context.get_mut()) },
      Context::Data(ref mut s) => unsafe { sass_delete_data_context(s.context.get_mut()) },
    };
  }
}
