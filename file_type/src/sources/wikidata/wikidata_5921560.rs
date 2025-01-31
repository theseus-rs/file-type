use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_5921560: FileFormat = FileFormat {
    id: 5_921_560,
    puid: "wikidata/5921560",
    name: "Synthetic music mobile application format",
    extensions: &["m3f", "mmf", "mqf"],
    media_types: &[
        "application/vnd.smaf",
        "application/vnd.smaf",
        "application/vnd.smaf",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
