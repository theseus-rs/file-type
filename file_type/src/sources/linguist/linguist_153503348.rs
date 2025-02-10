use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_153503348: FileType = FileType {
    file_format: &FileFormat {
        id: 153_503_348,
        source_type: SourceType::Linguist,
        name: "Browserslist",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
