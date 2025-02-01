use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110039733: FileFormat = FileFormat {
    id: 110_039_733,
    puid: "wikidata/110039733",
    name: "Mar Archive",
    extensions: &["mac", "mar"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
