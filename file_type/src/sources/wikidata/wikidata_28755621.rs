use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28755621: FileFormat = FileFormat {
    id: 28_755_621,
    puid: "wikidata/28755621",
    name: "Exact Yearbook ID file",
    extensions: &["id"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
