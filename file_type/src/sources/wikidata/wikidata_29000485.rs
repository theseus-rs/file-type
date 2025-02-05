use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29000485: FileFormat = FileFormat {
    id: 29_000_485,
    source_type: SourceType::Wikidata,
    name: "010 Editor Binary Template",
    extensions: &["bt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
