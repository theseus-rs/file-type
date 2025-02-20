use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
