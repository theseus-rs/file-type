use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51954390: FileFormat = FileFormat {
    id: 51_954_390,
    puid: "wikidata/51954390",
    name: "WordStar for MS-DOS Document, version 6",
    extensions: &["ws", "ws6"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
