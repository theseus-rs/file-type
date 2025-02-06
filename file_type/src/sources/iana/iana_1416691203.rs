use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1416691203: FileFormat = FileFormat {
    id: 1_416_691_203,
    source_type: SourceType::Iana,
    name: "CDFX+XML",
    extensions: &[],
    media_types: &["application/CDFX+XML"],
    signatures: &[],
    related_formats: &[],
};
