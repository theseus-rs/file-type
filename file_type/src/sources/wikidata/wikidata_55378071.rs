use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_55378071: FileFormat = FileFormat {
    id: 55_378_071,
    source_type: SourceType::Wikidata,
    name: "Marvin Document format",
    extensions: &["mrv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
