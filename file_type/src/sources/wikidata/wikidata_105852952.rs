use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852952: FileFormat = FileFormat {
    id: 105_852_952,
    puid: "wikidata/105852952",
    name: "A'dam Music Composer Script (with rem)",
    extensions: &["scr"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
