use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59630618: FileFormat = FileFormat {
    id: 59_630_618,
    puid: "wikidata/59630618",
    name: "Scriptware Script Format",
    extensions: &["sw3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
