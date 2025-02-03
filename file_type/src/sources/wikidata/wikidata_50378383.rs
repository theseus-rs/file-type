use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_50378383: FileFormat = FileFormat {
    id: 50_378_383,
    source_type: SourceType::Wikidata,
    name: "INTERLIS Transfer File, version 1",
    extensions: &["itf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
