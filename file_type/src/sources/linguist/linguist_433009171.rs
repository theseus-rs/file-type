use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_433009171: FileType = FileType {
    file_format: &FileFormat {
        id: 433_009_171,
        source_type: SourceType::Linguist,
        name: "Answer Set Programming",
        extensions: &["lp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
