use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_991047534: FileType = FileType {
    file_format: &FileFormat {
        id: 991_047_534,
        source_type: SourceType::Linguist,
        name: "Pro*C",
        extensions: &["pc"],
        media_types: &["text/x-csrc"],
        signatures: &[],
        related_formats: &[],
    },
};
