use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3445346377631037340: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "pg format",
    extensions: &["str"],
    media_types: &["application/vnd.pg.format"],
    internal_signatures: &[],
    related_formats: &[],
};
