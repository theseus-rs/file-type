use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_71859354: FileFormat = FileFormat {
    id: 71_859_354,
    puid: "wikidata/71859354",
    name: "CorelDraw Drawing, version 12",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
