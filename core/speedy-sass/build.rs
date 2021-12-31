use std::ffi::OsString;
use std::path::Path;
use std::process::Command;

///
/// 计算项目的全局目录
/// path example -> src/lib.rs
///
fn path_resolve(path: &str) -> String {
  let work_cwd = env!("CARGO_MANIFEST_DIR");
  let os_work_cwd = OsString::from(work_cwd);
  return Path::new(&os_work_cwd)
    .join(path)
    .into_os_string()
    .into_string()
    .unwrap();
}

/**
 * 批量增加 rust 构建中 查询静态库的 路径 include
 */
fn build_include_path(static_files: Vec<String>) {
  for static_path in static_files.iter() {
    println!("cargo:rustc-link-search=native={}", static_path);
  }
}

/**
 * 批量制定找寻的静态库 库名 libxx 则 -> xx 忽略前缀 lib
 */
fn build_include_dll(static_libs: Vec<String>) {
  for static_lib in static_libs.iter() {
    println!("cargo:rustc-link-lib=static={}", static_lib);
  }
}

///
/// 递归获取 路径文件夹下的 所有文件
///
fn get_dir_files(path: &str) -> Vec<String> {
  let mut filelist: Vec<String> = Vec::new();
  let mut files = std::fs::read_dir(path).unwrap();
  while let Some(file) = files.next() {
    let dir_entry = file.unwrap();
    let filename: String = dir_entry.file_name().into_string().unwrap();
    let name = format!("{}/{}", path, filename);
    let abs_name = path_resolve(&name);
    if dir_entry.file_type().unwrap().is_dir() {
      let temp_list = get_dir_files(&abs_name);
      filelist = [filelist.as_slice(), temp_list.as_slice()].concat();
    } else {
      filelist.push(abs_name);
    }
  }
  return filelist;
}

///
/// 通用方法
/// bindgen 生成定义文件
///
fn generate(
  headerpath: Vec<String>,
  outpath: &str,
  include_path: Option<Vec<&str>>,
  needcplus: Option<bool>,
) {
  let mut bindings = bindgen::Builder::default()
    .enable_cxx_namespaces()
    .generate_inline_functions(true)
    .derive_default(true);
  let arch = std::env::consts::ARCH;
  let arch_args = match arch {
    "aarch64" => "arm64",
    _ => "x86_64",
  };
  bindings = bindings.clang_arg("-arch").clang_arg(arch_args);
  for header in headerpath.into_iter() {
    bindings = bindings.header(header);
  }
  if let Some(c_headerdir_path) = include_path {
    let include_path_list = c_headerdir_path;
    for path in include_path_list.iter() {
      bindings = bindings.clang_arg("-I").clang_arg(path.to_string());
    }
  }
  if needcplus.unwrap_or(false) {
    bindings = bindings.clang_arg("-x c++").clang_arg("-std=c++11");
  }

  let res = bindings
    .layout_tests(false)
    .rustfmt_bindings(true)
    .detect_include_paths(true)
    .generate_comments(true)
    .generate()
    .unwrap();

  res.write_to_file(outpath).unwrap();
}

///
/// 加载 libsass clib 如果存在则 拉去git更新 如果不存在 则拉去git仓库
/// 然后进行 makefile 编译操作 变成 *.a 静态库
///
fn load_compile_clib(source_path: &str, lib_path: &str, clib_file_path: &str) {
  if !Path::new(lib_path).exists() {
    // 拉去 git 仓库
    let git_url = "git@github.com:sass/libsass.git";
    let mut task = Command::new("git");
    task.arg("clone").arg(git_url);
    task.current_dir(source_path);
    task
      .status()
      .expect(&format!("git clone {} --> failed", git_url));
  } else {
    // 更新 git 仓库
    let mut task = Command::new("git");
    task.arg("pull");
    task.current_dir(lib_path);
    task.status().expect("source/FFmpeg 下执行 git pull 失败");
  }
  if !Path::new(clib_file_path).exists() {
    // 执行编译
    let mut task = Command::new("make");
    task.current_dir(lib_path);
    task.status().expect("source/FFmpeg 下执行 git pull 失败");
  }
}

///
/// rust 编译时 携带 c语言编译好的静态库
/// 类似 编译 typescript 携带本有的 js 文件
///
fn rs_compile(clib_include_path: &str) {
  build_include_path(vec![clib_include_path.to_string()]);
  build_include_dll(vec!["sass".to_string()]);
}

///
/// 调用 通用方法 生成 rs 类型文件 类似 *.d.ts
///
fn gen(headers_path: &str, output_file: &str) {
  let headers = get_dir_files(headers_path);
  generate(headers, output_file, Some(vec![headers_path]), None);
}

///
/// 构建项目 代码
///
fn main() {
  let source_path = path_resolve("source");
  let lib_path = path_resolve("source/libsass");
  let sass_clib_path = path_resolve("source/libsass/lib");
  let sass_clib_file_path = path_resolve("source/libsass/lib/libsass.a");
  let sass_clib_headers_path = path_resolve("source/libsass/include");
  let sass_type_rs_dir = path_resolve("src/sass_clib.rs");

  // 加载 libsass
  load_compile_clib(&source_path, &lib_path, &sass_clib_file_path);
  rs_compile(&sass_clib_path);
  gen(&sass_clib_headers_path, &sass_type_rs_dir);
  println!(".....")
}
