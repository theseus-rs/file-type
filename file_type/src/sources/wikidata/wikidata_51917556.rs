use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51917556: FileFormat = FileFormat {
    id: 51_917_556,
    puid: "wikidata/51917556",
    name: "WordStar for Windows Document",
    extensions: &["ws", "wsd", "wsw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
