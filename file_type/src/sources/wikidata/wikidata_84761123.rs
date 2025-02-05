use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_84761123: FileFormat = FileFormat {
    id: 84_761_123,
    source_type: SourceType::Wikidata,
    name: ".gn",
    extensions: &["gn"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
