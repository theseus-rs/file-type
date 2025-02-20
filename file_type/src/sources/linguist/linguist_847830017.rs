use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_847830017: FileType = FileType {
    file_format: &FileFormat {
        id: 847_830_017,
        source_type: SourceType::Linguist,
        name: "sed",
        extensions: &["sed"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
