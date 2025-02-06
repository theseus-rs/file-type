use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130456404: FileFormat = FileFormat {
    id: 130_456_404,
    source_type: SourceType::Wikidata,
    name: "Beancount fileformat",
    extensions: &["beancount"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
