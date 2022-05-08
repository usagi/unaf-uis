/// uis ビルドプリプロセス
#[allow(dead_code)]
fn main()
{
 use build_pretty::{
  build_pretty,
  CommandBuilder,
  BUILD_TYPE_STRING
 };
 use const_format::concatcp;

 const PUB_PATH: &str = concatcp!("pub.", BUILD_TYPE_STRING);

 const RESOUCE_INCLUDABLE_PATH: &str = ".build/resource.includable.rs";
 const BUILD_PUB_COMMAND: &str = "cargo";
 const BUILD_PUB_CARGO_COMMAND: &str = "make";

 const BUILD_PUB_CARGO_MAKE_COMMAND: &str = concatcp!("build-", BUILD_TYPE_STRING, "-pub");

 const BUILD_PUB_ARGS: &[&str] = &[BUILD_PUB_CARGO_COMMAND, BUILD_PUB_CARGO_MAKE_COMMAND];

 const TASK_BUILD_PUB: &str = concatcp!("src ➟ ", PUB_PATH);
 const TASK_COLLECT_PUB: &str = concatcp!(PUB_PATH, " ➟ ", RESOUCE_INCLUDABLE_PATH);

 // 1. src -> PUB_PATH への uis 本体のビルドプロセスを実行
 build_pretty!().enque_command(
  TASK_BUILD_PUB,
  CommandBuilder::new_with_args(BUILD_PUB_COMMAND, BUILD_PUB_ARGS).into()
 )
 // 2. build/resource.includable.rs を更新
 .enque_fn(TASK_COLLECT_PUB, Box::new(|output| {

  use glob::glob;
  use std::fmt::Write;
  const PATTERN: &str = concatcp!(PUB_PATH, "/**/*");
  output.write_fmt(format_args!("PATTERN = {PATTERN:?}\n"))?;
  let mut entries = vec![];
  for entry in glob(PATTERN)?
  {
   match entry
   {
    Ok(path) if path.is_file() => {
     let path = path.strip_prefix(PUB_PATH)?;
     entries.push(format!("(\"{p}\".to_string(),include_bytes!(\"../{PUB_PATH}/{p}\").to_vec())", p=path.display()));
     output.write_fmt(format_args!("uis.lib ⬅ {:?}\n", path.display()))?
    },
    Err(e) => eprintln!("{:?}", e),
    _ => ()
   }
  }
  let body = format!("DictionaryType::from([{}])", entries.join(",\n"));
  std::fs::write(RESOUCE_INCLUDABLE_PATH, body)?;
  Ok(())
 }));
}
