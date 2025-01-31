use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_71432876: FileFormat = FileFormat {
    id: 71_432_876,
    puid: "wikidata/71432876",
    name: "CorelDraw Drawing, version 6",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
