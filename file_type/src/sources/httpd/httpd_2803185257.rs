use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2803185257: FileFormat = FileFormat {
    id: 2_803_185_257,
    source_type: SourceType::Httpd,
    name: "cdmi queue",
    extensions: &["cdmiq"],
    media_types: &["application/cdmi-queue"],
    internal_signatures: &[],
    related_formats: &[],
};
