use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113557545: FileFormat = FileFormat {
    id: 113_557_545,
    source_type: SourceType::Wikidata,
    name: "Gear Image",
    extensions: &["p01"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
