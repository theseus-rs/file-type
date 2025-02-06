use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3109186030: FileFormat = FileFormat {
    id: 3_109_186_030,
    source_type: SourceType::Iana,
    name: "BV32",
    extensions: &[],
    media_types: &["audio/BV32"],
    signatures: &[],
    related_formats: &[],
};
