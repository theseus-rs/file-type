use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_155357471: FileType = FileType {
    file_format: &FileFormat {
        id: 155_357_471,
        source_type: SourceType::Linguist,
        name: "Pod 6",
        extensions: &["pod", "pod6"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
