use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_52426038: FileFormat = FileFormat {
    id: 52_426_038,
    puid: "wikidata/52426038",
    name: "WordStar for MS-DOS Document, version 3",
    extensions: &["ws", "ws3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
