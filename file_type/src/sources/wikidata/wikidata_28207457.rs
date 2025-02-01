use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207457: FileFormat = FileFormat {
    id: 28_207_457,
    puid: "wikidata/28207457",
    name: "Vista Plain File Format",
    extensions: &["vp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
