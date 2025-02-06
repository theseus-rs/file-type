use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50378320: FileFormat = FileFormat {
    id: 50_378_320,
    source_type: SourceType::Wikidata,
    name: "INTERLIS Transfer File, version 2.2",
    extensions: &["xml", "xtf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
