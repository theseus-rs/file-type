use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_48694183: FileFormat = FileFormat {
    id: 48_694_183,
    puid: "wikidata/48694183",
    name: "WordStar for MS-DOS Document, version 7",
    extensions: &["ws", "ws7"],
    media_types: &["application/x-wordstar", "application/x-wordstar"],
    internal_signatures: &[],
    related_formats: &[],
};
