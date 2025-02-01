use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27349984: FileFormat = FileFormat {
    id: 27_349_984,
    puid: "wikidata/27349984",
    name: "TOPSAR Correlation Map",
    extensions: &["corgr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
