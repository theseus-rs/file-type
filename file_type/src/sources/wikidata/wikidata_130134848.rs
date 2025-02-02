use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130134848: FileFormat = FileFormat {
    id: 130_134_848,
    source_type: SourceType::Wikidata,
    name: "Kusto query file",
    extensions: &["kql"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
