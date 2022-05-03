use time::{error::Format, format_description, macros::format_description, PrimitiveDateTime};

pub const DATE_FMT: &str = "[day padding:none] [month repr:long] [year]";
pub const DATETIME_FMT: &str =
    "[day padding:none] [month repr:long] [year] [hour]:[minute]:[second]";

pub struct Datetime;

impl Datetime {
    pub fn format(date_str: &str, format_desc: &str) -> Result<String, Format> {
        let primitive_date_fmt =
            format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
        let primitive_date = PrimitiveDateTime::parse(date_str, &primitive_date_fmt).unwrap();
        let display_fmt = format_description::parse(&format_desc).unwrap();
        return primitive_date.format(&display_fmt);
    }
}
