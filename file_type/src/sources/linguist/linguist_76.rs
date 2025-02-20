use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_76: FileType = FileType {
    file_format: &FileFormat {
        id: 76,
        source_type: SourceType::Linguist,
        name: "Gherkin",
        extensions: &["feature", "story"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
