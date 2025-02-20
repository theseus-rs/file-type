use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_840483232: FileType = FileType {
    file_format: &FileFormat {
        id: 840_483_232,
        source_type: SourceType::Linguist,
        name: "Open Policy Agent",
        extensions: &["rego"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
