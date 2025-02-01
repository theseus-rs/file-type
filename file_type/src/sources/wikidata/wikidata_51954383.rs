use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51954383: FileFormat = FileFormat {
    id: 51_954_383,
    puid: "wikidata/51954383",
    name: "WordStar for MS-DOS Document, version 5.5",
    extensions: &["ws"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
