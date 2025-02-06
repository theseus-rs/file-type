use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125822910: FileFormat = FileFormat {
    id: 125_822_910,
    source_type: SourceType::Wikidata,
    name: "Compiled HTML Help index file",
    extensions: &["chw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
