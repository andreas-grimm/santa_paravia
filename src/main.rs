/**
  Name: paravia.rs
  Description:This is a port of the original TRS-80 BASIC code for Santa Paravia and Fiumaccio, 
  (C) 1979 George Blank (used with permission).
  By: Thomas Knox                                                               

  Inputs:N/A                                                                    

  Returns:N/A                                                                   

  Assumes:Should compile and run on any system with an Rust compiler.         

  Side Effects:N/A                                                              

  This code is copyrighted and has limited warranties.
*/
mod text;
mod common;

use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  let mut query: String = "-text".parse().unwrap();

  if args.len() > 1 {
    query = args[1].clone();
  }

  // import line: file::module::function
  text::santa_paravia_module::santa_paravia_text();
}
