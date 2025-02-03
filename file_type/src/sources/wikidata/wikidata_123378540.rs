use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123378540: FileFormat = FileFormat {
    id: 123_378_540,
    source_type: SourceType::Wikidata,
    name: "Light library file",
    extensions: &["lgl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
