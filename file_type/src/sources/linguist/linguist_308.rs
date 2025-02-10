use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_308: FileType = FileType {
    file_format: &FileFormat {
        id: 308,
        source_type: SourceType::Linguist,
        name: "RAML",
        extensions: &["raml"],
        media_types: &["text/x-yaml"],
        signatures: &[],
        related_formats: &[],
    },
};
