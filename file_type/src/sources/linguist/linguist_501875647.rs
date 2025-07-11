use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_501875647: FileType = FileType {
    file_format: &FileFormat {
        id: 501_875_647,
        source_type: SourceType::Linguist,
        name: "ReScript",
        extensions: &["res", "resi"],
        media_types: &["text/x-rustsrc"],
        signatures: &[],
        related_formats: &[],
    },
};
