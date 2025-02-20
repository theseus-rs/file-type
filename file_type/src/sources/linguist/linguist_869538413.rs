use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_869538413: FileType = FileType {
    file_format: &FileFormat {
        id: 869_538_413,
        source_type: SourceType::Linguist,
        name: "Reason",
        extensions: &["re", "rei"],
        media_types: &["text/x-rustsrc"],
        signatures: &[],
        related_formats: &[],
    },
};
