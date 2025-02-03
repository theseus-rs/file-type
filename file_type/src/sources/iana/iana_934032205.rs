use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_934032205: FileFormat = FileFormat {
    id: 934_032_205,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.presentationml.template.main+xml",
    extensions: &[],
    media_types: &[
        "application/vnd.openxmlformats-officedocument.presentationml.template.main+xml",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
