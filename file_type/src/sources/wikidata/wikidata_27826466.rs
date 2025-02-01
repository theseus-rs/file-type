use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27826466: FileFormat = FileFormat {
    id: 27_826_466,
    puid: "wikidata/27826466",
    name: "Cascading Style Sheets Level 2",
    extensions: &["css"],
    media_types: &["text/css"],
    internal_signatures: &[],
    related_formats: &[],
};
