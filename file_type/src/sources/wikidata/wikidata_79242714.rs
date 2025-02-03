use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_79242714: FileFormat = FileFormat {
    id: 79_242_714,
    source_type: SourceType::Wikidata,
    name: "Lotus Approach Database index",
    extensions: &["adx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
