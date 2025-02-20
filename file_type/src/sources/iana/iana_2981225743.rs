use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2981225743: FileType = FileType {
    file_format: &FileFormat {
        id: 2_981_225_743,
        source_type: SourceType::Iana,
        name: "tamp-apex-update-confirm",
        extensions: &[],
        media_types: &["application/tamp-apex-update-confirm"],
        signatures: &[],
        related_formats: &[],
    },
};
