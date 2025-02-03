use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29000485: FileFormat = FileFormat {
    id: 29_000_485,
    source_type: SourceType::Wikidata,
    name: "010 Editor Binary Template",
    extensions: &["bt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
