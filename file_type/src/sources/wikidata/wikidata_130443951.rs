use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130443951: FileFormat = FileFormat {
    id: 130_443_951,
    source_type: SourceType::Wikidata,
    name: "Subsembly JSON",
    extensions: &["json"],
    media_types: &["application/json"],
    signatures: &[],
    related_formats: &[],
};
