use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_101250905: FileFormat = FileFormat {
    id: 101_250_905,
    puid: "wikidata/101250905",
    name: ".piskel",
    extensions: &["piskel"],
    media_types: &["image/png+json"],
    internal_signatures: &[],
    related_formats: &[],
};
