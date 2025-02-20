use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_506780613: FileType = FileType {
    file_format: &FileFormat {
        id: 506_780_613,
        source_type: SourceType::Linguist,
        name: "Nextflow",
        extensions: &["nf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
