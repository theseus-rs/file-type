use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1737548538: FileType = FileType {
    file_format: &FileFormat {
        id: 1_737_548_538,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.presentationml.notesSlide+xml",
        extensions: &[],
        media_types: &[
            "application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
