use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3191008066: FileFormat = FileFormat {
    id: 3_191_008_066,
    source_type: SourceType::Httpd,
    name: "shana informed formdata",
    extensions: &["ifm"],
    media_types: &["application/vnd.shana.informed.formdata"],
    signatures: &[],
    related_formats: &[],
};
