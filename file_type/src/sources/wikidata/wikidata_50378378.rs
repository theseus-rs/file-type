use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_50378378: FileFormat = FileFormat {
    id: 50_378_378,
    source_type: SourceType::Wikidata,
    name: "INTERLIS Model File, version 2.2",
    extensions: &["ili"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
