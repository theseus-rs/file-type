use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130443951: FileFormat = FileFormat {
    id: 130_443_951,
    source_type: SourceType::Wikidata,
    name: "Subsembly JSON",
    extensions: &["json"],
    media_types: &["application/json"],
    internal_signatures: &[],
    related_formats: &[],
};
