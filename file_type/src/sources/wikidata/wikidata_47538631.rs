use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47538631: FileFormat = FileFormat {
    id: 47_538_631,
    puid: "wikidata/47538631",
    name: "AutoCAD Custom Dictionary",
    extensions: &["cus"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
