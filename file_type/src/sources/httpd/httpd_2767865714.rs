use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2767865714: FileFormat = FileFormat {
    id: 2_767_865_714,
    source_type: SourceType::Httpd,
    name: "hp jlyt",
    extensions: &["jlt"],
    media_types: &["application/vnd.hp-jlyt"],
    signatures: &[],
    related_formats: &[],
};
