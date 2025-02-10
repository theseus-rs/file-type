use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_135: FileType = FileType {
    file_format: &FileFormat {
        id: 135,
        source_type: SourceType::Linguist,
        name: "Grace",
        extensions: &["grace"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
