use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_71275233: FileFormat = FileFormat {
    id: 71_275_233,
    puid: "wikidata/71275233",
    name: "CorelDraw Drawing, version 7",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
