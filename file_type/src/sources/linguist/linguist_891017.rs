use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_891017: FileType = FileType {
    file_format: &FileFormat {
        id: 891_017,
        source_type: SourceType::Linguist,
        name: "LiveCode Script",
        extensions: &["livecodescript"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
