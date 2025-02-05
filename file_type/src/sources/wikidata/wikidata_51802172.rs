use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51802172: FileFormat = FileFormat {
    id: 51_802_172,
    source_type: SourceType::Wikidata,
    name: "Speller Custom Dictionary",
    extensions: &["dic"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
