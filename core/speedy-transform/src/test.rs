#[cfg(test)]
pub mod tests {
  use crate::types::*;
  use crate::web_transform::parser::transform;

  fn compare_handle(str: std::string::String) -> std::string::String {
    let new_str = str.trim().replace("\n", "");
    return new_str;
  }

  #[test]
  fn env() {
    for (key, val) in std::env::vars() {
      println!("{:?} is {:?} ....", key, val);
    }
  }

  #[test]
  fn babel_import_test() {
    let source = "
import React from \"react\";
import ReactDOM from \"react-dom\";
import {Button, Input} from \"antd\";
import Child from \"./component/Child\";

class Page extends React.Component<any,any> {
    render() {
        return (
            <div className={\"test\"}>
                <div>Page</div>
                <Child/>
                <Button>click me</Button>
                <Input/>
            </div>
        );
    }
}

ReactDOM.render(<Page/>, document.getElementById(\"root\"));
";

    let target_code = "
import \"antd/es/input/style/index.css\";
import \"antd/es/button/style/index.css\";
import Input from \"antd/es/input/index.js\";
import Button from \"antd/es/button/index.js\";
import React from \"react\";
import ReactDOM from \"react-dom\";
import Child from \"./component/Child\";
class Page extends React.Component {
    render() {
        return (<div className={\"test\"}>

                <div >Page</div>

                <Child />

                <Button >click me</Button>

                <Input />

            </div>);
    }
}
ReactDOM.render(<Page />, document.getElementById(\"root\"));
";

    let transfrom_res = transform(
      source,
      TransformConfig {
        react_runtime: Some(true),
        babel_import: Some(vec![BabelImportConfig {
          from_source: "antd".to_string(),
          replace_css: Some(RepalceCssConfig {
            ignore_style_component: None,
            replace_expr: "antd/es/{}/style/index.css".to_string(),
            lower: Some(true),
          }),
          replace_js: Some(RepalceSpecConfig {
            ignore_es_component: None,
            replace_expr: "antd/es/{}/index.js".to_string(),
            lower: Some(true),
          }),
        }]),
      },
      None,
      Some("ES5".to_string()),
    )
    .unwrap();
    assert_eq!(format!("\n{}", transfrom_res.code), target_code);
  }

  #[test]
  fn react_perfix_test() {
    let source = "\
import { Button, Input} from \"antd\";
const a = 123;
    ";
    let target_code = "
import React from \"react\";
import { Button, Input } from \"antd\";
const a = 123;
";
    let transfrom_res = transform(
      source,
      TransformConfig {
        react_runtime: Some(true),
        babel_import: None,
      },
      None,
      Some("ES5".to_string()),
    )
    .unwrap();
    assert_eq!(format!("\n{}", transfrom_res.code), target_code);
  }

  #[test]
  fn swc_all_test() {
    let source = "\
import { useState, useCallback, useEffect, Fragment } from 'react';
import { Image } from '@byted-growth/luckycat-mobile';
import { throttle } from '@byted-growth/luckycat-util';

const a = 123;
    ";
    let transfrom_res = transform(
      source,
      TransformConfig {
        react_runtime: Some(false),
        babel_import: Some(vec![
          BabelImportConfig {
            from_source: "@byted-growth/luckycat-mobile".to_string(),
            replace_css: Some(RepalceCssConfig {
              ignore_style_component: Some(vec![
                "Image".to_string(),
                "ConfigProvider".to_string(),
                "ConfigContext".to_string(),
              ]),
              replace_expr: "@byted-growth/luckycat-mobile/theme/components/{}/style/index.css"
                .to_string(),
              lower: Some(false),
            }),
            replace_js: Some(RepalceSpecConfig {
              ignore_es_component: None,
              replace_expr: "@byted-growth/luckycat-mobile/es/{}/index.js".to_string(),
              lower: Some(false),
            }),
          },
          BabelImportConfig {
            from_source: "@byted-growth/luckycat-util".to_string(),
            replace_css: None,
            replace_js: Some(RepalceSpecConfig {
              ignore_es_component: None,
              replace_expr: "@byted-growth/luckycat-util/pure_es/{}/index.js".to_string(),
              lower: Some(false),
            }),
          },
        ]),
      },
      None,
      Some("ES5".to_string()),
    )
    .unwrap();
    let target_code = "\
import throttle from \"@byted-growth/luckycat-util/pure_es/throttle/index.js\";
import Image from \"@byted-growth/luckycat-mobile/es/Image/index.js\";
import { useState, useCallback, useEffect, Fragment } from 'react';
const a = 123;
    ";
    assert_eq!(
      compare_handle(transfrom_res.code),
      compare_handle(target_code.to_string())
    );
  }
}
