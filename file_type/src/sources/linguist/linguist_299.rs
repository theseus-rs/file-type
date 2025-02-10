use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_299: FileType = FileType {
    file_format: &FileFormat {
        id: 299,
        source_type: SourceType::Linguist,
        name: "Puppet",
        extensions: &["pp"],
        media_types: &["text/x-puppet"],
        signatures: &[],
        related_formats: &[],
    },
};
