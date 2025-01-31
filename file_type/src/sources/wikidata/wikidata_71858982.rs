use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_71858982: FileFormat = FileFormat {
    id: 71_858_982,
    puid: "wikidata/71858982",
    name: "CorelDraw Drawing, version 10",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
