use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_128034054: FileFormat = FileFormat {
    id: 128_034_054,
    source_type: SourceType::Wikidata,
    name: "Rebol script",
    extensions: &["r"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
