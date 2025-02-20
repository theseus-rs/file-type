use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_288: FileType = FileType {
    file_format: &FileFormat {
        id: 288,
        source_type: SourceType::Linguist,
        name: "Pod",
        extensions: &["pod"],
        media_types: &["text/x-perl"],
        signatures: &[],
        related_formats: &[],
    },
};
