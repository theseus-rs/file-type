use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113533133: FileFormat = FileFormat {
    id: 113_533_133,
    source_type: SourceType::Wikidata,
    name: "LegalDocML Document",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
