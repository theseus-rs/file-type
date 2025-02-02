use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125255905: FileFormat = FileFormat {
    id: 125_255_905,
    source_type: SourceType::Wikidata,
    name: "Simulation Result File",
    extensions: &["mat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
