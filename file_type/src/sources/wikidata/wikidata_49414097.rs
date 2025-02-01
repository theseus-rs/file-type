use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_49414097: FileFormat = FileFormat {
    id: 49_414_097,
    puid: "wikidata/49414097",
    name: "CATIA Model, version 4",
    extensions: &["mod", "model"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
