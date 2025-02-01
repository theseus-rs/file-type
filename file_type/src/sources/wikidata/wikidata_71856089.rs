use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_71856089: FileFormat = FileFormat {
    id: 71_856_089,
    puid: "wikidata/71856089",
    name: "CorelDraw Drawing, version 9",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
