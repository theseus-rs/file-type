use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50378383: FileFormat = FileFormat {
    id: 50_378_383,
    source_type: SourceType::Wikidata,
    name: "INTERLIS Transfer File, version 1",
    extensions: &["itf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
