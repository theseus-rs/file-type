use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_341496207: FileFormat = FileFormat {
    id: 341_496_207,
    source_type: SourceType::Httpd,
    name: "ecmascript",
    extensions: &["ecma"],
    media_types: &["application/ecmascript"],
    internal_signatures: &[],
    related_formats: &[],
};
