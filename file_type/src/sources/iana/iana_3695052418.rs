use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3695052418: FileType = FileType {
    file_format: &FileFormat {
        id: 3_695_052_418,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.spreadsheetml.template.main+xml",
        extensions: &[],
        media_types: &[
            "application/vnd.openxmlformats-officedocument.spreadsheetml.template.main+xml",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
