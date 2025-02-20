use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_306: FileType = FileType {
    file_format: &FileFormat {
        id: 306,
        source_type: SourceType::Linguist,
        name: "QMake",
        extensions: &["pri", "pro"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
