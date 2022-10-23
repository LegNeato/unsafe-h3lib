#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]
#![feature(register_tool)]
#![register_tool(c2rust)]
#![allow(clippy::missing_safety_doc)]

extern crate unsafe_h3lib;
extern crate unsafe_h3lib_applib;

// Symbols in these binaries are used by other test binaries.
pub mod testH3CellArea;
pub mod testH3Memory;
