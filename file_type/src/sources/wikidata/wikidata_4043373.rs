use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_4043373: FileFormat = FileFormat {
    id: 4_043_373,
    source_type: SourceType::Wikidata,
    name: "MAGMA",
    extensions: &["magma"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
