use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_825782436: FileFormat = FileFormat {
    id: 825_782_436,
    source_type: SourceType::Iana,
    name: "wasm",
    extensions: &[],
    media_types: &["application/wasm"],
    internal_signatures: &[],
    related_formats: &[],
};
