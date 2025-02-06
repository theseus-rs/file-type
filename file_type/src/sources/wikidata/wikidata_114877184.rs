use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114877184: FileFormat = FileFormat {
    id: 114_877_184,
    source_type: SourceType::Wikidata,
    name: "Money 95, 97, and 98 Backup File",
    extensions: &["mny"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
