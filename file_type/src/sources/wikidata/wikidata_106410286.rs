use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_106410286: FileFormat = FileFormat {
    id: 106_410_286,
    puid: "wikidata/106410286",
    name: "ZISRAW (CZI)",
    extensions: &["czi"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
