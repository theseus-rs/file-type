use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_10397010: FileFormat = FileFormat {
    id: 10_397_010,
    source_type: SourceType::Wikidata,
    name: ".rmp",
    extensions: &["rmp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
