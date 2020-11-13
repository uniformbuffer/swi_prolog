#[derive(Clone,Debug,PartialEq)]
pub enum Data
{
    String(String),
    Bool(bool),
    U32(u32),
    U64(u64),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64)
}


impl From<String> for Data {fn from(value: String) -> Self {Self::String(value)}}
impl From<&str> for Data {fn from(value: &str) -> Self {Self::String(value.to_string())}}
impl From<bool> for Data {fn from(value: bool) -> Self {Self::Bool(value)}}
impl From<u32> for Data {fn from(value: u32) -> Self {Self::U32(value)}}
impl From<u64> for Data {fn from(value: u64) -> Self {Self::U64(value)}}
impl From<i32> for Data {fn from(value: i32) -> Self {Self::I32(value)}}
impl From<i64> for Data {fn from(value: i64) -> Self {Self::I64(value)}}
impl From<f32> for Data {fn from(value: f32) -> Self {Self::F32(value)}}
impl From<f64> for Data {fn from(value: f64) -> Self {Self::F64(value)}}
