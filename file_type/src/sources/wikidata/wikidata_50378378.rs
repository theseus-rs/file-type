use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_50378378: FileFormat = FileFormat {
    id: 50_378_378,
    puid: "wikidata/50378378",
    name: "INTERLIS Model File, version 2.2",
    extensions: &["ili"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
