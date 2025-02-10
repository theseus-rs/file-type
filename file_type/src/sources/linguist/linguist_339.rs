use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_339: FileType = FileType {
    file_format: &FileFormat {
        id: 339,
        source_type: SourceType::Linguist,
        name: "SaltStack",
        extensions: &["sls"],
        media_types: &["text/x-yaml"],
        signatures: &[],
        related_formats: &[],
    },
};
