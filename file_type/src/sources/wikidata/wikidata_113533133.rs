use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113533133: FileFormat = FileFormat {
    id: 113_533_133,
    source_type: SourceType::Wikidata,
    name: "LegalDocML Document",
    extensions: &["xml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
