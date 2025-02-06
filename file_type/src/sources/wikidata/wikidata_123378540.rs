use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123378540: FileFormat = FileFormat {
    id: 123_378_540,
    source_type: SourceType::Wikidata,
    name: "Light library file",
    extensions: &["lgl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
