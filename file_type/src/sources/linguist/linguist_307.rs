use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_307: FileType = FileType {
    file_format: &FileFormat {
        id: 307,
        source_type: SourceType::Linguist,
        name: "R",
        extensions: &["r", "rd", "rsx"],
        media_types: &["text/x-rsrc"],
        signatures: &[],
        related_formats: &[],
    },
};
