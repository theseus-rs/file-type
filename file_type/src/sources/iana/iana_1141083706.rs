use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1141083706: FileFormat = FileFormat {
    id: 1_141_083_706,
    source_type: SourceType::Iana,
    name: "simpleSymbolContainer",
    extensions: &[],
    media_types: &["application/simpleSymbolContainer"],
    signatures: &[],
    related_formats: &[],
};
