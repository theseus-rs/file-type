use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_572649: FileFormat = FileFormat {
    id: 572_649,
    source_type: SourceType::Wikidata,
    name: "Intel HEX",
    extensions: &["hex"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
