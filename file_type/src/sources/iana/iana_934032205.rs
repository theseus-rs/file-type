use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_934032205: FileType = FileType {
    file_format: &FileFormat {
        id: 934_032_205,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.presentationml.template.main+xml",
        extensions: &[],
        media_types: &[
            "application/vnd.openxmlformats-officedocument.presentationml.template.main+xml",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
