use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_64859030: FileFormat = FileFormat {
    id: 64_859_030,
    puid: "wikidata/64859030",
    name: "Family Tree Maker for Windows file format",
    extensions: &["ftw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
