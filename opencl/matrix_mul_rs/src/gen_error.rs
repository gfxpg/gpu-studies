use std::{io, num, error::Error, fmt, fmt::{Display, Formatter}};
use ocl;
use ocl_core;

#[macro_export]
macro_rules! with_gen_error {
    ($e:expr) => ($e.map_err(|e| GenError::from(e)))
}

#[macro_export]
macro_rules! gen_error_format {
    ($($args:expr),*) => (Err(GenError::from(format!($($args),*))))
}

#[macro_export]
macro_rules! unwrap {
    ($e:expr) => (match $e {
        Ok(val) => val,
        Err(err) => { eprintln!("{}", err); process::exit(1); }
    })
}

pub type GenResult<T> = Result<T, GenError>;

#[derive(Debug)]
pub struct GenError {
    message: String
}

impl Display for GenError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for GenError {
    fn description(&self) -> &str {
        self.message.as_str()
    }
}

impl<'a> From<&'a str> for GenError {
    fn from(s: &'a str) -> Self {
        GenError { message: s.to_owned() }
    }
}

impl From<String> for GenError {
    fn from(s: String) -> Self {
        GenError { message: s }
    }
}

macro_rules! impl_from_as_to_string {
    ($type:ty) => {
        impl From<$type> for GenError {
            fn from(tinst: $type) -> Self { GenError { message: tinst.to_string() } }
        }
    }
}

impl_from_as_to_string!(io::Error);
impl_from_as_to_string!(num::ParseFloatError);
impl_from_as_to_string!(ocl::Error);
impl_from_as_to_string!(ocl_core::error::Error);
