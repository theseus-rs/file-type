use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_572649: FileFormat = FileFormat {
    id: 572_649,
    source_type: SourceType::Wikidata,
    name: "Intel HEX",
    extensions: &["hex"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
