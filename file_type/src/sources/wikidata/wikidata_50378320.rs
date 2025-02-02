use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_50378320: FileFormat = FileFormat {
    id: 50_378_320,
    source_type: SourceType::Wikidata,
    name: "INTERLIS Transfer File, version 2.2",
    extensions: &["xml", "xtf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
