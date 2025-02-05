use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123360066: FileFormat = FileFormat {
    id: 123_360_066,
    source_type: SourceType::Wikidata,
    name: "PHP 4 Web Page",
    extensions: &["php4"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
