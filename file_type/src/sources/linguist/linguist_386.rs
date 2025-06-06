use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_386: FileType = FileType {
    file_format: &FileFormat {
        id: 386,
        source_type: SourceType::Linguist,
        name: "Vala",
        extensions: &["vala", "vapi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
