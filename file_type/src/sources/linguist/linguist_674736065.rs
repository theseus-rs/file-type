use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_674736065: FileType = FileType {
    file_format: &FileFormat {
        id: 674_736_065,
        source_type: SourceType::Linguist,
        name: "robots.txt",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
