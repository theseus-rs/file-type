use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1663033906: FileType = FileType {
    file_format: &FileFormat {
        id: 1_663_033_906,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.wordprocessingml.document.glossary+xml",
        extensions: &[],
        media_types: &[
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document.glossary+xml",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
