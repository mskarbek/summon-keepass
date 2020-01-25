extern crate ini;
extern crate keepass;

use ini::Ini;
use keepass::{Database, Node, Result};
use std::env;
use std::fs::File;
use std::process;
use std::io::{self, Write};

fn main() -> Result<()> {
    let stdout = io::stdout();
    let mut out_handle = stdout.lock();
    let stderr = io::stderr();
    let mut err_handle = stderr.lock();

    let args: Vec<_> = env::args_os().collect();
    if args.len() <= 1 {
        err_handle.write(b"no variable was provided").unwrap();
        err_handle.flush().unwrap();
        process::exit(1);
    }

    let config_path = format!("{}/.summon-keepass.ini", env::var("HOME").unwrap());

    let config = Ini::load_from_file(config_path.as_str()).unwrap();
    let keepass_db = config.section(Some("keepass_db".to_owned())).unwrap();
    let keepass_db_path = keepass_db.get("path").unwrap();
    let keepass_db_pass = keepass_db.get("pass").unwrap();

    let pass_path = args[1].to_str().unwrap().split("/").collect::<Vec<&str>>();

    let path = std::path::Path::new(keepass_db_path);
    let db = Database::open(&mut File::open(path)?, Some(keepass_db_pass), None)?;

    if let Some(Node::Entry(e)) = db.root.get(&pass_path) {
	out_handle.write(e.get_password().unwrap().as_bytes()).unwrap();
	out_handle.flush().unwrap();
	process::exit(0);
    }
    err_handle.write(format!("{} could not be retrieved", args[1].to_str().unwrap()).as_bytes()).unwrap();
    err_handle.flush().unwrap();
    process::exit(1);
}
