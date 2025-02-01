use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_10394820: FileFormat = FileFormat {
    id: 10_394_820,
    puid: "wikidata/10394820",
    name: "Zope Configuration Markup Language",
    extensions: &["zcml"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
