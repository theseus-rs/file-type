use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_162: FileType = FileType {
    file_format: &FileFormat {
        id: 162,
        source_type: SourceType::Linguist,
        name: "IGOR Pro",
        extensions: &["ipf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
