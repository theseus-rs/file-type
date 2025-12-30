use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_852099832: FileType = FileType {
    file_format: &FileFormat {
        id: 852_099_832,
        source_type: SourceType::Linguist,
        name: "KoLmafia ASH",
        extensions: &["ash"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
