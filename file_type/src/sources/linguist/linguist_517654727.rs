use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_517654727: FileType = FileType {
    file_format: &FileFormat {
        id: 517_654_727,
        source_type: SourceType::Linguist,
        name: "mIRC Script",
        extensions: &["mrc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
