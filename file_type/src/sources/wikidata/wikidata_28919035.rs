use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28919035: FileFormat = FileFormat {
    id: 28_919_035,
    puid: "wikidata/28919035",
    name: "Type-1 DV AVI",
    extensions: &["avi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
