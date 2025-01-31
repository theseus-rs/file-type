use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_119973449: FileFormat = FileFormat {
    id: 119_973_449,
    puid: "wikidata/119973449",
    name: "WebEasy Web Document",
    extensions: &["web"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
