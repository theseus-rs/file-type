use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28755628: FileFormat = FileFormat {
    id: 28_755_628,
    puid: "wikidata/28755628",
    name: "Exact Yearbook LST file",
    extensions: &["lst"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
