use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_791486801: FileFormat = FileFormat {
    id: 791_486_801,
    source_type: SourceType::Httpd,
    name: "exi",
    extensions: &["exi"],
    media_types: &["application/exi"],
    internal_signatures: &[],
    related_formats: &[],
};
