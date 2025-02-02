use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_84761123: FileFormat = FileFormat {
    id: 84_761_123,
    source_type: SourceType::Wikidata,
    name: ".gn",
    extensions: &["gn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
