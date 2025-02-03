use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2596484608: FileFormat = FileFormat {
    id: 2_596_484_608,
    source_type: SourceType::Httpd,
    name: "3ds",
    extensions: &["3ds"],
    media_types: &["image/x-3ds"],
    internal_signatures: &[],
    related_formats: &[],
};
