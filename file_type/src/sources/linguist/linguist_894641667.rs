use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_894641667: FileType = FileType {
    file_format: &FileFormat {
        id: 894_641_667,
        source_type: SourceType::Linguist,
        name: "Slice",
        extensions: &["ice"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
