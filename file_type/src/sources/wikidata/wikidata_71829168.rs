use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_71829168: FileFormat = FileFormat {
    id: 71_829_168,
    puid: "wikidata/71829168",
    name: "CorelDraw Drawing, version 3",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
