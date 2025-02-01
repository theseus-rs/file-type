use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61912820: FileFormat = FileFormat {
    id: 61_912_820,
    puid: "wikidata/61912820",
    name: "ODM",
    extensions: &["odm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
