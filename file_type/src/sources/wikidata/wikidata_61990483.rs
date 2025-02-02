use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_61990483: FileFormat = FileFormat {
    id: 61_990_483,
    source_type: SourceType::Wikidata,
    name: "Log ASCII Standard Format, version 2",
    extensions: &["las"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
