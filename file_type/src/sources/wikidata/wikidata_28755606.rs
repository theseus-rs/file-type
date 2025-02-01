use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28755606: FileFormat = FileFormat {
    id: 28_755_606,
    puid: "wikidata/28755606",
    name: "Exact Yearbook DAT file",
    extensions: &["dat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
