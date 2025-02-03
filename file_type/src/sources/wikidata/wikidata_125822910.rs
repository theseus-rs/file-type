use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125822910: FileFormat = FileFormat {
    id: 125_822_910,
    source_type: SourceType::Wikidata,
    name: "Compiled HTML Help index file",
    extensions: &["chw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
