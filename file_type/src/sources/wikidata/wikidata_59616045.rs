use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59616045: FileFormat = FileFormat {
    id: 59_616_045,
    puid: "wikidata/59616045",
    name: "Zope export file",
    extensions: &["zexp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
