use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2369373394: FileType = FileType {
    file_format: &FileFormat {
        id: 2_369_373_394,
        source_type: SourceType::Iana,
        name: "vnd.blockfact.factv",
        extensions: &[],
        media_types: &["video/vnd.blockfact.factv"],
        signatures: &[],
        related_formats: &[],
    },
};
