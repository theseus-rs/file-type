use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29000647: FileFormat = FileFormat {
    id: 29_000_647,
    source_type: SourceType::Wikidata,
    name: "PLG",
    extensions: &["plg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
