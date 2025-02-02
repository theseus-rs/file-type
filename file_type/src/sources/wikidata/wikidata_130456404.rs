use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130456404: FileFormat = FileFormat {
    id: 130_456_404,
    source_type: SourceType::Wikidata,
    name: "Beancount fileformat",
    extensions: &["beancount"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
