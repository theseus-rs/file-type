use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2230729471: FileFormat = FileFormat {
    id: 2_230_729_471,
    source_type: SourceType::Httpd,
    name: "grafeq",
    extensions: &["gqf", "gqs"],
    media_types: &["application/vnd.grafeq"],
    internal_signatures: &[],
    related_formats: &[],
};
