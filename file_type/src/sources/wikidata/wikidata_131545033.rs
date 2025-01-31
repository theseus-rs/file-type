use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131545033: FileFormat = FileFormat {
    id: 131_545_033,
    puid: "wikidata/131545033",
    name: "Stanford Exploration Project file",
    extensions: &["h"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
