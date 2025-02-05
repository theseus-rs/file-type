use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1586807912: FileFormat = FileFormat {
    id: 1_586_807_912,
    source_type: SourceType::Httpd,
    name: "msmediaview",
    extensions: &["mvb", "m13", "m14"],
    media_types: &["application/x-msmediaview"],
    signatures: &[],
    related_formats: &[],
};
