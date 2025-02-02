use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114877188: FileFormat = FileFormat {
    id: 114_877_188,
    source_type: SourceType::Wikidata,
    name: "Money 1.0, 2.0, and 3.0 Backup File",
    extensions: &["bak"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
