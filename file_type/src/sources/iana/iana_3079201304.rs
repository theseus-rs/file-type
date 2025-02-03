use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3079201304: FileFormat = FileFormat {
    id: 3_079_201_304,
    source_type: SourceType::Iana,
    name: "vnd.oma-scws-http-response",
    extensions: &[],
    media_types: &["application/vnd.oma-scws-http-response"],
    internal_signatures: &[],
    related_formats: &[],
};
