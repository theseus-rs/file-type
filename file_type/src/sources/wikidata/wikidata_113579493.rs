use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113579493: FileFormat = FileFormat {
    id: 113_579_493,
    source_type: SourceType::Wikidata,
    name: "Justfile",
    extensions: &["just", "justfile"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
