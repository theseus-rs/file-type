use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127433402: FileFormat = FileFormat {
    id: 127_433_402,
    puid: "wikidata/127433402",
    name: "Smalltalk Source Code",
    extensions: &["st"],
    media_types: &["text/x-smalltalk"],
    internal_signatures: &[],
    related_formats: &[],
};
