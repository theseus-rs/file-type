use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29650340: FileFormat = FileFormat {
    id: 29_650_340,
    source_type: SourceType::Wikidata,
    name: "PES",
    extensions: &["pes"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
