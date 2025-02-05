use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27826464: FileFormat = FileFormat {
    id: 27_826_464,
    source_type: SourceType::Wikidata,
    name: "Cascading Style Sheets Level 1",
    extensions: &["css"],
    media_types: &["text/css"],
    signatures: &[],
    related_formats: &[],
};
