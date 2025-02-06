use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1045587053: FileFormat = FileFormat {
    id: 1_045_587_053,
    source_type: SourceType::Iana,
    name: "BV16",
    extensions: &[],
    media_types: &["audio/BV16"],
    signatures: &[],
    related_formats: &[],
};
