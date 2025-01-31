use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850422: FileFormat = FileFormat {
    id: 105_850_422,
    puid: "wikidata/105850422",
    name: "16bit DOS COM DS-COM Crypt protected (v1.27)",
    extensions: &["com"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
