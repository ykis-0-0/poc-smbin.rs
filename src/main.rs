use std::env;

use eyre::Result as eResult;

mod cli;
use cli::Cli;

mod logger;
use logger::setup_log;

use log::{ error, warn, info, debug, trace };
use object::{Object, ObjectSection};

// #[repr(C)]
#[used]
static mut TESTER: u32 = 0;

fn main() -> eResult<()> {
  let args = <Cli as clap::Parser>::parse();

  color_eyre::install()?;
  setup_log(2 /* args.get_verbosity() */)?;

  println!("Hello, world!");

  // Locate myself
  let exe = env::current_exe()?;
  debug!("My path is: [{:?}]", &exe);

  // Read myself as an object file
  let bytes = std::fs::read(&exe)?;
  let object_file = object::File::parse(bytes.as_slice())?;

  // Get Contents
  for sect in object_file.sections() {
    println!("{:?}", sect.name());
  }

  println!("{:?}", exe);

  /*
  - find the exe
  - open as obj file
  - get contents
  - write
  */

  return Ok(());

  /*
  match args.opcode {
    cli::OpCode::Get => todo!(),
    cli::OpCode::Clear => todo!(),
    cli::OpCode::Measure => unimplemented!(),
    cli::OpCode::Add => todo!(),
    cli::OpCode::New => unimplemented!(),
  }
  */
}
