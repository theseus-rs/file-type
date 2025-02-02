use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123360595: FileFormat = FileFormat {
    id: 123_360_595,
    source_type: SourceType::Wikidata,
    name: "PHP 5 Web Page",
    extensions: &["php5"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
