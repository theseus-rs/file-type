use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849853: FileFormat = FileFormat {
    id: 105_849_853,
    puid: "wikidata/105849853",
    name: "16bit DOS COM DS-COM Crypt protected (v1.31)",
    extensions: &["com"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
