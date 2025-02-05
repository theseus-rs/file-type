use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29000737: FileFormat = FileFormat {
    id: 29_000_737,
    source_type: SourceType::Wikidata,
    name: "Volume data format",
    extensions: &["vol"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
