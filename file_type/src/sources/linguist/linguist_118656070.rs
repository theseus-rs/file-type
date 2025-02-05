use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_118656070: FileFormat = FileFormat {
    id: 118_656_070,
    source_type: SourceType::Linguist,
    name: "TL-Verilog",
    extensions: &["tlv"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
