use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_71859176: FileFormat = FileFormat {
    id: 71_859_176,
    puid: "wikidata/71859176",
    name: "CorelDraw Drawing, version 11",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
