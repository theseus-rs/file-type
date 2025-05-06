use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_597930447: FileType = FileType {
    file_format: &FileFormat {
        id: 597_930_447,
        source_type: SourceType::Linguist,
        name: "Koka",
        extensions: &["kk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
