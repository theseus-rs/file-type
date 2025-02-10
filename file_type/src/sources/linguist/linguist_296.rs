use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_296: FileType = FileType {
    file_format: &FileFormat {
        id: 296,
        source_type: SourceType::Linguist,
        name: "Propeller Spin",
        extensions: &["spin"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
