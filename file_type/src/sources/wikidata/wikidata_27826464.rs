use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27826464: FileFormat = FileFormat {
    id: 27_826_464,
    source_type: SourceType::Wikidata,
    name: "Cascading Style Sheets Level 1",
    extensions: &["css"],
    media_types: &["text/css"],
    internal_signatures: &[],
    related_formats: &[],
};
