use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_10397010: FileFormat = FileFormat {
    id: 10_397_010,
    puid: "wikidata/10397010",
    name: ".rmp",
    extensions: &["rmp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
