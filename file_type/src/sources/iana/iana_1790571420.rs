use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1790571420: FileType = FileType {
    file_format: &FileFormat {
        id: 1_790_571_420,
        source_type: SourceType::Iana,
        name: "vnd.project-graph",
        extensions: &[],
        media_types: &["application/vnd.project-graph"],
        signatures: &[],
        related_formats: &[],
    },
};
