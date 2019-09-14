use std::{error::Error, fs, path::PathBuf};

use rlua::{Lua, MultiValue, ToLua};
use structopt::StructOpt;

mod lua_api;

use lua_api::Remodel;

#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(parse(from_os_str))]
    script: PathBuf,

    script_arguments: Vec<String>,
}

fn start() -> Result<(), Box<dyn Error>> {
    let opt = Options::from_args();
    let contents = fs::read_to_string(&opt.script)?;
    let lua = Lua::new();

    lua.context(move |context| {
        let lua_args = opt
            .script_arguments
            .into_iter()
            .map(|value| value.to_lua(context))
            .collect::<Result<Vec<_>, _>>()?;

        context.globals().set("remodel", Remodel)?;

        let chunk = context.load(&contents);
        chunk.call(MultiValue::from_vec(lua_args))
    })?;

    Ok(())
}

fn main() {
    match start() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
