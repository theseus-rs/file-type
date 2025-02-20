use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3019298279: FileType = FileType {
    file_format: &FileFormat {
        id: 3_019_298_279,
        source_type: SourceType::Iana,
        name: "vnd.nokia.landmarkcollection+xml",
        extensions: &[],
        media_types: &["application/vnd.nokia.landmarkcollection+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
