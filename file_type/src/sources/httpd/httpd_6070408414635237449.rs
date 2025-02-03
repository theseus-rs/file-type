use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_6070408414635237449: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "stepmania package",
    extensions: &["smzip"],
    media_types: &["application/vnd.stepmania.package"],
    internal_signatures: &[],
    related_formats: &[],
};
