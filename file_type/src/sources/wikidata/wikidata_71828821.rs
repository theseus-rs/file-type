use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_71828821: FileFormat = FileFormat {
    id: 71_828_821,
    puid: "wikidata/71828821",
    name: "CorelDraw Drawing, version 4",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
