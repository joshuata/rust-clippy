#![feature(rustc_private)]
#![feature(box_syntax)]

extern crate syntax;
#[macro_use]
extern crate rustc;
extern crate rustc_plugin;
extern crate datalog;

pub mod test_analyzer;

use std::path::Path;

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn register_plugins(reg: &mut rustc_plugin::Registry) {
    let writer = datalog::writer::FileWriter::new(Path::new("test.datalog"));
    reg.register_early_lint_pass(box test_analyzer::TestAnalyzer::new(writer));
}
