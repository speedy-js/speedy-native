// use speedy_transform::web_transform::{
//   parser::transform_module,
//   proxy::{ExtraInfo, TransformConfig},
// };
// use swc_plugin::{ast::Program, metadata::TransformPluginProgramMetadata, plugin_transform};

// #[plugin_transform]
// pub fn speedy_transform(program: Program, metadata: TransformPluginProgramMetadata) -> Program {
//   let config_str = metadata.get_transform_plugin_config();
//   let config =
//     serde_json::from_str::<TransformConfig>(&config_str.unwrap_or("{}".to_string())).unwrap();

//   let mut cloned = program.clone();
//   if let Program::Module(module) = &mut cloned {
//     transform_module(module, &config, &ExtraInfo);
//   }

//   cloned
// }
