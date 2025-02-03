use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_5904094350807371736: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "bmi",
    extensions: &["bmi"],
    media_types: &["application/vnd.bmi"],
    internal_signatures: &[],
    related_formats: &[],
};
