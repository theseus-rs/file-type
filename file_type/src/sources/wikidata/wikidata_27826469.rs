use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27826469: FileFormat = FileFormat {
    id: 27_826_469,
    puid: "wikidata/27826469",
    name: "Cascading Style Sheets Level 3",
    extensions: &["css"],
    media_types: &["text/css"],
    internal_signatures: &[],
    related_formats: &[],
};
