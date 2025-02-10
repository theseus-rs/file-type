use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_447261135: FileType = FileType {
    file_format: &FileFormat {
        id: 447_261_135,
        source_type: SourceType::Linguist,
        name: "JAR Manifest",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
