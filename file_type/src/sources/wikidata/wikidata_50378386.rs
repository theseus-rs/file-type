use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_50378386: FileFormat = FileFormat {
    id: 50_378_386,
    puid: "wikidata/50378386",
    name: "INTERLIS Model File, version 1",
    extensions: &["ili"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
