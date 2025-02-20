use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_405: FileType = FileType {
    file_format: &FileFormat {
        id: 405,
        source_type: SourceType::Linguist,
        name: "Xojo",
        extensions: &[
            "xojo_code",
            "xojo_menu",
            "xojo_report",
            "xojo_script",
            "xojo_toolbar",
            "xojo_window",
        ],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
