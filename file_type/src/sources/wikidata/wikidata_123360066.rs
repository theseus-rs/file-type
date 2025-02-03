use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123360066: FileFormat = FileFormat {
    id: 123_360_066,
    source_type: SourceType::Wikidata,
    name: "PHP 4 Web Page",
    extensions: &["php4"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
