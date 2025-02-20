use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3151184436: FileType = FileType {
    file_format: &FileFormat {
        id: 3_151_184_436,
        source_type: SourceType::Iana,
        name: "vnd.marlin.drm.conftoken+xml",
        extensions: &[],
        media_types: &["application/vnd.marlin.drm.conftoken+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
