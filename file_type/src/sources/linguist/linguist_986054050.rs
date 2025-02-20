use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_986054050: FileType = FileType {
    file_format: &FileFormat {
        id: 986_054_050,
        source_type: SourceType::Linguist,
        name: "Genero 4gl",
        extensions: &["4gl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
