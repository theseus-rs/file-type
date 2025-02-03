use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_10846539: FileFormat = FileFormat {
    id: 10_846_539,
    source_type: SourceType::Wikidata,
    name: "BNA",
    extensions: &["bna"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
