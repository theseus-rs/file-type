use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27826464: FileFormat = FileFormat {
    id: 27_826_464,
    puid: "wikidata/27826464",
    name: "Cascading Style Sheets Level 1",
    extensions: &["css"],
    media_types: &["text/css"],
    internal_signatures: &[],
    related_formats: &[],
};
