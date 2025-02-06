use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_4042481: FileFormat = FileFormat {
    id: 4_042_481,
    source_type: SourceType::Wikidata,
    name: "LOGML",
    extensions: &["xml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
