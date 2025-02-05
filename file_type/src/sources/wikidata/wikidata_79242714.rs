use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_79242714: FileFormat = FileFormat {
    id: 79_242_714,
    source_type: SourceType::Wikidata,
    name: "Lotus Approach Database index",
    extensions: &["adx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
