use crate::utils::time::now_string;

pub const INFO: &str = "\x1b[37m\x1b[44m\x1b[1mINFO\x1b[0m";
pub const CONN: &str = "\x1b[37m\x1b[43m\x1b[1mCONN\x1b[0m";
pub const DEBUG: &str = "\x1b[37m\x1b[100m\x1b[1mDBUG\x1b[0m";
pub const ERROR: &str = "\x1b[37m\x1b[41m\x1b[1mEROR\x1b[0m";
pub const CRIT: &str = "\x1b[37m\x1b[41m\x1b[1mCRIT\x1b[0m";

pub fn info(str: &str) {

    println!("{}", info_string(str));

}

fn info_string(str: &str) -> String {

    format!("{INFO} {} {}", now_string(), str)

}

pub fn conn(str: &str) {

    println!("{}", conn_string(str));

}

fn conn_string(str: &str) -> String {

    format!("{CONN} {} {}", now_string(), str)

}

pub fn debug(str: &str) {

    println!("{}", debug_string(str));

}

fn debug_string(str: &str) -> String {

    format!("{DEBUG} {} {}", now_string(), str)

}

pub fn error(str: &str) {

    println!("{}", error_string(str));

}

fn error_string(str: &str) -> String {

    format!("{ERROR} {} {}", now_string(), str)

}

pub fn critical(str: &str) {

    println!("{}", critical_string(str));

}

pub fn critical_panic(str: &str) -> ! {

    panic!("{}", critical_string(str));

}

fn critical_string(str: &str) -> String {

    format!("{ERROR} {} {}", now_string(), str)

}
