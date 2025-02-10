use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3409486063: FileType = FileType {
    file_format: &FileFormat {
        id: 3_409_486_063,
        source_type: SourceType::Iana,
        name: "vnd.dcmp+xml",
        extensions: &[],
        media_types: &["application/vnd.dcmp+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
