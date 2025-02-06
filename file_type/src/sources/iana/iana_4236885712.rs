use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4236885712: FileFormat = FileFormat {
    id: 4_236_885_712,
    source_type: SourceType::Iana,
    name: "asc",
    extensions: &[],
    media_types: &["audio/asc"],
    signatures: &[],
    related_formats: &[],
};
