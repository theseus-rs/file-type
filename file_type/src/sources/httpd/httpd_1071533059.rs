use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1071533059: FileFormat = FileFormat {
    id: 1_071_533_059,
    source_type: SourceType::Httpd,
    name: "stepmania package",
    extensions: &["smzip"],
    media_types: &["application/vnd.stepmania.package"],
    signatures: &[],
    related_formats: &[],
};
