use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130134848: FileFormat = FileFormat {
    id: 130_134_848,
    source_type: SourceType::Wikidata,
    name: "Kusto query file",
    extensions: &["kql"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
