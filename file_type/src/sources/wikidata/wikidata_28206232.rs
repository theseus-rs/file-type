use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206232: FileFormat = FileFormat {
    id: 28_206_232,
    source_type: SourceType::Wikidata,
    name: "HP Paintjet",
    extensions: &["pjx1"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
