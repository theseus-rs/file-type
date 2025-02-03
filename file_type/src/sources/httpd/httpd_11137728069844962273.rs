use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_11137728069844962273: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "yamaha smaf audio",
    extensions: &["saf"],
    media_types: &["application/vnd.yamaha.smaf-audio"],
    internal_signatures: &[],
    related_formats: &[],
};
