use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_619814037: FileType = FileType {
    file_format: &FileFormat {
        id: 619_814_037,
        source_type: SourceType::Linguist,
        name: "Scenic",
        extensions: &["scenic"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
