use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123436713: FileFormat = FileFormat {
    id: 123_436_713,
    source_type: SourceType::Wikidata,
    name: "Single-Molecule Dataset file",
    extensions: &["smd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
