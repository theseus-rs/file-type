use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114078864: FileFormat = FileFormat {
    id: 114_078_864,
    puid: "wikidata/114078864",
    name: "MacBinary II",
    extensions: &["bin"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
