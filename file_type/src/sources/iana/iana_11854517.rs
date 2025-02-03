use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_11854517: FileFormat = FileFormat {
    id: 11_854_517,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.presentationml.presentation.main+xml",
    extensions: &[],
    media_types: &[
        "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
