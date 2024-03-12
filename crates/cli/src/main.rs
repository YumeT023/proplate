use clap::{arg, Command};
use cmd::{
  create::{create, CreateOptions},
  init::init,
};

mod cmd;

fn cli() -> Command {
  const VERSION: &str = env!("CARGO_PKG_VERSION");
  Command::new("proplate")
    .version(VERSION)
    .author("Yume Saiko <yumii.saiko@gmail.com>")
    .about(
      r#"
    ▄▄▄▄▄▄▄ ▄▄▄▄▄▄   ▄▄▄▄▄▄▄ ▄▄▄▄▄▄▄ ▄▄▄     ▄▄▄▄▄▄ ▄▄▄▄▄▄▄ ▄▄▄▄▄▄▄ 
    █       █   ▄  █ █       █       █   █   █      █       █       █
    █    ▄  █  █ █ █ █   ▄   █    ▄  █   █   █  ▄   █▄     ▄█    ▄▄▄█
    █   █▄█ █   █▄▄█▄█  █ █  █   █▄█ █   █   █ █▄█  █ █   █ █   █▄▄▄ 
    █    ▄▄▄█    ▄▄  █  █▄█  █    ▄▄▄█   █▄▄▄█      █ █   █ █    ▄▄▄█
    █   █   █   █  █ █       █   █   █       █  ▄   █ █   █ █   █▄▄▄ 
    █▄▄▄█   █▄▄▄█  █▄█▄▄▄▄▄▄▄█▄▄▄█   █▄▄▄▄▄▄▄█▄█ █▄▄█ █▄▄▄█ █▄▄▄▄▄▄▄█
    
Any Project starter in one tool"#,
    )
    .subcommands([
      Command::new("create")
        .about("create project from template")
        .args(&[
          arg!(--template <template> "Template location to start from (dir or github repo url)")
            .required(true),
          arg!(--dest <dest> "Destination path").required(true),
          arg!(--git  "Initialize git repo"),
        ]),
      Command::new("init").about("initialize a template").args(&[
        arg!(<id> "id for the template").required(true),
        arg!(--dest <dest> "Destination path, if not set dir with 'id' will be created"),
      ]),
    ])
}

fn main() -> Result<(), clap::Error> {
  let matches = cli().get_matches();
  let subcommands = matches.subcommand();

  match subcommands {
    Some(cmd) => match cmd {
      ("create", args) => {
        let template_id = args.get_one::<String>("template").unwrap().as_str();
        let dest = args.get_one::<String>("dest").unwrap().as_str();

        let options = CreateOptions {
          git: args.get_flag("git"),
          ..Default::default()
        };

        create(template_id, dest, options).expect(
          format!(
            "Unable to create boilerplate from Template [{}]",
            template_id
          )
          .as_str(),
        )
      }
      ("init", args) => {
        let id = args.get_one::<String>("id").unwrap();
        let dest = args.get_one::<String>("dest");
        init(String::from(id), dest.map(|e| e.into()))
          .expect(format!("Unable to initialize template [id={}]", id,).as_str());
      }
      _ => {}
    },
    _ => cli().print_help()?,
  }

  Ok(())
}
