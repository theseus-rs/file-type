use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_350026389: FileType = FileType {
    file_format: &FileFormat {
        id: 350_026_389,
        source_type: SourceType::Iana,
        name: "MP4V-ES",
        extensions: &[],
        media_types: &["video/MP4V-ES"],
        signatures: &[],
        related_formats: &[],
    },
};
