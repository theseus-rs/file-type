use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114877184: FileFormat = FileFormat {
    id: 114_877_184,
    source_type: SourceType::Wikidata,
    name: "Money 95, 97, and 98 Backup File",
    extensions: &["mny"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
