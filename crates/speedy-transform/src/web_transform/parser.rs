use crate::types::TransformConfig;
use crate::web_transform::babel_import::transform_style;
use crate::web_transform::react::transform_perfixreact;
use swc::config::SourceMapsConfig;
use swc::{Compiler, TransformOutput};
use swc_common::input::StringInput;
use swc_common::sync::Lrc;
use swc_common::{FileName, SourceMap};
use swc_ecma_ast::EsVersion;
use swc_ecma_parser::lexer::Lexer;
use swc_ecma_parser::{Parser, Syntax, TsConfig};

pub fn transform(
  code: &str,
  config: TransformConfig,
  filename: Option<String>,
  target: Option<String>,
) -> Result<TransformOutput, std::string::String> {
  let source_filename = filename.unwrap_or_else(|| "test.js".to_string());
  let cm: Lrc<SourceMap> = Default::default();
  let fm = cm.new_source_file(FileName::Custom(source_filename.clone()), code.into());
  let compiler = Compiler::new(cm);

  let lexer = Lexer::new(
    // We want to parse ecmascript
    Syntax::Typescript(TsConfig {
      tsx: true,
      decorators: true,
      dts: false,
      no_early_errors: false,
    }),
    // EsVersion defaults to es5
    EsVersion::Es2016,
    StringInput::from(&*fm),
    None,
  );

  let mut parser = Parser::new_from(lexer);

  let list_error = parser.take_errors();
  if list_error.iter().len() > 0 {
    let err_msg = list_error
      .iter()
      .map(|err| err.kind().msg())
      .collect::<Vec<_>>()
      .join("");
    return Err(err_msg);
  }

  let module_reuslt = parser.parse_module();
  if module_reuslt.is_err() {
    return Err(module_reuslt.err().unwrap().into_kind().msg().to_string());
  }
  let mut module = module_reuslt.unwrap();

  transform_style(&mut module, &config);
  transform_perfixreact(&mut module, &config, code);

  let target_ref = target.unwrap_or_else(|| "".to_string());
  let swc_target = match target_ref.as_str() {
    "" => EsVersion::Es5,
    "ES5" => EsVersion::Es5,
    "ES6" => EsVersion::Es2015,
    _ => EsVersion::Es2020,
  };

  // let handler = Handler::with_emitter_writer(Box::new(stderr()), Some(compiler.cm.clone()));
  //
  // let mut buf = vec![];
  // {
  //   let mut emitter = Emitter {
  //     cfg: swc_ecma_codegen::Config { minify: false },
  //     cm: compiler.cm.clone(),
  //     comments: None,
  //     wr: JsWriter::new(compiler.cm.clone(), "\n", &mut buf, None),
  //   };
  //   emitter.emit_module(&module).unwrap();
  // }
  //
  // println!("u8 -> {}", std::str::from_utf8(&buf).unwrap());

  // let opt = Options {
  //   is_module: IsModule::Bool(true),
  //   config: Config {
  //     env: Some(Default::default()),
  //     module: Some(ModuleConfig::CommonJs(Default::default())),
  //     jsc: JscConfig {
  //       syntax: Some(Syntax::Typescript(TsConfig {
  //         tsx: true,
  //         decorators: true,
  //         ..Default::default()
  //       })),
  //       transform: Some(swc::config::TransformConfig {
  //         legacy_decorator: true,
  //         ..Default::default()
  //       }),
  //       ..Default::default()
  //     },
  //     ..Default::default()
  //   },
  //   ..Default::default()
  // };
  //
  // let res = compiler.process_js_file(fm, &handler, &opt);

  // print!("{}", res.unwrap().code);

  compiler
    .print(
      &module,
      Some(source_filename.as_str()),
      None,
      false,
      swc_target,
      SourceMapsConfig::Bool(true),
      &Default::default(),
      None,
      false,
      None,
    )
    .map_err(|err| err.to_string())
}
