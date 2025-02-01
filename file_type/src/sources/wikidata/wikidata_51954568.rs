use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51954568: FileFormat = FileFormat {
    id: 51_954_568,
    puid: "wikidata/51954568",
    name: "WordStar for Windows Document, version 2",
    extensions: &["ws", "wsw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
