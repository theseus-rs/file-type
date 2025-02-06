use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1946098896: FileFormat = FileFormat {
    id: 1_946_098_896,
    source_type: SourceType::Iana,
    name: "vnd.yamaha.hv-script",
    extensions: &[],
    media_types: &["application/vnd.yamaha.hv-script"],
    signatures: &[],
    related_formats: &[],
};
