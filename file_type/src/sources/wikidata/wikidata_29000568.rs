use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29000568: FileFormat = FileFormat {
    id: 29_000_568,
    source_type: SourceType::Wikidata,
    name: "Tacx Fortius",
    extensions: &["caf", "imf", "mrlv", "pgmf", "rlv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
