use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3268047355: FileFormat = FileFormat {
    id: 3_268_047_355,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml",
    extensions: &[],
    media_types: &[
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml",
    ],
    signatures: &[],
    related_formats: &[],
};
