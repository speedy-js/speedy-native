extern crate proc_macro;

use proc_macro::*;
use std::string::String;

#[proc_macro_attribute]
pub fn speedydebugtest(_args: TokenStream, input: TokenStream) -> TokenStream {
  println!("final:{:#?}", input.to_string());
  input.to_string().parse().unwrap()
}

#[proc_macro_attribute]
pub fn speedydebug(_args: TokenStream, input: TokenStream) -> TokenStream {
  let res_tuple = rs_fn_parser(input.to_string().as_str());
  // println!("item: \"{:#?}\"", res_tuple);
  let args_str = res_tuple.1.clone();
  let mut arg_var_name_list: Vec<String> = vec![];
  args_str
    .split(',')
    .collect::<Vec<&str>>()
    .into_iter()
    .for_each(|kv| {
      let varname = match kv.split(':').collect::<Vec<&str>>().get(0) {
        Some(&name) => name,
        _ => "",
      };
      if !varname.is_empty() {
        arg_var_name_list.push("&".to_string() + varname.trim());
      }
    });

  let mut fn_content = res_tuple.2.clone();
  let macro_debug_content_template = "\
  let needexec = {
    let res = std::env::var(\"rsdebug\");
    match res {
        Ok(val) => {
          if val == \"info\" {
            true
          } else {
            false
          }
        }
        _ => false,
      }
    };
  if needexec {
    let args = @@@;
    let json_content = serde_json::to_string_pretty(&args).unwrap();
    let profile_path = std::env::current_dir()
                        .unwrap()
                        .join(\"rsprofile.json\")
                        .into_os_string()
                        .into_string()
                        .unwrap();
    std::fs::write(profile_path.as_str(), json_content).unwrap();
  }
  ";
  fn_content = macro_debug_content_template.replace(
    "@@@",
    format!("(\"{}\",{})", res_tuple.0, arg_var_name_list.join(",")).as_str(),
  ) + fn_content.as_str();
  let new_fn_str = format!(
    "{} fn {}({}){}{}{}{}",
    res_tuple.4, res_tuple.0, res_tuple.1, res_tuple.3, "{", fn_content, "}"
  );

  // println!("final:{:#?}", new_fn_str);
  new_fn_str.parse().unwrap()
}

fn rs_fn_parser(str: &str) -> (String, String, String, String, String) {
  let strtoarray = |str: &str| str.chars().map(|c| c.to_string()).collect::<Vec<String>>();
  let ignore_lastchar = |str: &str| {
    let mut chars = str.chars();
    chars.next_back();
    chars.as_str().to_string()
  };
  let intercept_str = drawstring(strtoarray(str).as_ref(), Some("fn"), Some("{"))
    .trim()
    .to_string();
  let fn_name = drawstring(strtoarray(intercept_str.as_str()).as_ref(), None, Some("("))
    .trim()
    .to_string();
  let args_str = drawstring(
    strtoarray(intercept_str.as_str()).as_ref(),
    Some("("),
    Some(")"),
  )
  .trim()
  .to_string();
  let fn_content_str =
    ignore_lastchar(drawstring(strtoarray(str).as_ref(), Some("{"), None).trim());
  let mut res_str = drawstring(strtoarray(str).as_ref(), Some("->"), Some("{"))
    .trim()
    .to_string();
  let macro_str = ignore_lastchar(drawstring(strtoarray(str).as_ref(), None, Some("fn")).trim());
  if res_str.as_str() != "" {
    res_str = " -> ".to_string() + res_str.as_str();
  }
  (fn_name, args_str, fn_content_str, res_str, macro_str)
}

fn drawstring(charlist: &[String], beginchar: Option<&str>, endchar: Option<&str>) -> String {
  let mut sindex = 0;
  let mut temp = "".to_string();
  let mut handle_scan_context: Vec<String> = vec![];
  let mut write_enable = false;
  if beginchar.is_none() {
    write_enable = true;
  }
  let safe_get = |list: &[String], index: usize| match list.get(index) {
    Some(char) => char.to_string(),
    _ => "".to_string(),
  };

  loop {
    let char = safe_get(charlist, sindex);
    temp += &char;
    if sindex >= charlist.len() {
      break;
    }
    if temp.as_str() != ""
      && beginchar.is_some()
      && temp.contains(beginchar.unwrap())
      && !write_enable
    {
      temp = "".to_string();
      write_enable = true;
      sindex += 1;
      continue;
    }
    if temp.as_str() != "" && endchar.is_some() && temp.contains(endchar.unwrap()) {
      break;
    }
    if write_enable {
      handle_scan_context.push(char.clone());
    }
    sindex += 1;
  }
  handle_scan_context.join("")
}
