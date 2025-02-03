use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123436713: FileFormat = FileFormat {
    id: 123_436_713,
    source_type: SourceType::Wikidata,
    name: "Single-Molecule Dataset file",
    extensions: &["smd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
