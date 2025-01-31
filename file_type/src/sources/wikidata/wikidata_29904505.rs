use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29904505: FileFormat = FileFormat {
    id: 29_904_505,
    puid: "wikidata/29904505",
    name: "S7z",
    extensions: &["s7z"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
