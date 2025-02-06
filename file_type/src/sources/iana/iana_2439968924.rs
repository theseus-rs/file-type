use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2439968924: FileFormat = FileFormat {
    id: 2_439_968_924,
    source_type: SourceType::Iana,
    name: "vnd.gml",
    extensions: &[],
    media_types: &["text/vnd.gml"],
    signatures: &[],
    related_formats: &[],
};
