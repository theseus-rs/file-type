use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_71274998: FileFormat = FileFormat {
    id: 71_274_998,
    puid: "wikidata/71274998",
    name: "CorelDraw Drawing, version 8",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
