use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_988547172: FileType = FileType {
    file_format: &FileFormat {
        id: 988_547_172,
        source_type: SourceType::Linguist,
        name: "Common Workflow Language",
        extensions: &["cwl"],
        media_types: &["text/x-yaml"],
        signatures: &[],
        related_formats: &[],
    },
};
