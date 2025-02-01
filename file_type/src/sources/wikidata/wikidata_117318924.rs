use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117318924: FileFormat = FileFormat {
    id: 117_318_924,
    puid: "wikidata/117318924",
    name: "WordPerfect Graphic 2.0",
    extensions: &["wp2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
