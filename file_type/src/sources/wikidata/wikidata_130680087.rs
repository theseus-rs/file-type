use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130680087: FileFormat = FileFormat {
    id: 130_680_087,
    source_type: SourceType::Wikidata,
    name: "Anvil",
    extensions: &["mca"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
