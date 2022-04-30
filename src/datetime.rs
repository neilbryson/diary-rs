use time::{error::Format, macros::format_description, PrimitiveDateTime, format_description};

pub struct Datetime;

impl Datetime {
    pub fn format(date_str: &str, format_desc: &str) -> Result<String, Format> {
        let primitive_date_fmt = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
        let primitive_date = PrimitiveDateTime::parse(date_str, &primitive_date_fmt).unwrap();
        let display_fmt = format_description::parse(&format_desc).unwrap();
        return primitive_date.format(&display_fmt);
    }
}
