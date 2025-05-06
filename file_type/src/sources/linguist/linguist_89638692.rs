use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_89638692: FileType = FileType {
    file_format: &FileFormat {
        id: 89_638_692,
        source_type: SourceType::Linguist,
        name: "M3U",
        extensions: &["m3u", "m3u8"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
