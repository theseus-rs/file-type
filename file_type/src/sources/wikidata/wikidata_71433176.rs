use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_71433176: FileFormat = FileFormat {
    id: 71_433_176,
    puid: "wikidata/71433176",
    name: "CorelDraw Drawing, version 5",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
