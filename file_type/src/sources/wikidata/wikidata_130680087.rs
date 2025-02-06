use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130680087: FileFormat = FileFormat {
    id: 130_680_087,
    source_type: SourceType::Wikidata,
    name: "Anvil",
    extensions: &["mca"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
