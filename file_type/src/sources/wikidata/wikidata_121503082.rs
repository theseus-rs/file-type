use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_121503082: FileFormat = FileFormat {
    id: 121_503_082,
    puid: "wikidata/121503082",
    name: "Image Systems CCITT Group 4 file",
    extensions: &["ig4"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
