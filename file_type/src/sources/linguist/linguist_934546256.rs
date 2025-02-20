use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_934546256: FileType = FileType {
    file_format: &FileFormat {
        id: 934_546_256,
        source_type: SourceType::Linguist,
        name: "Go Workspace",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
