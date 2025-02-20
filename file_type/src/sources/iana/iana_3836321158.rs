use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3836321158: FileType = FileType {
    file_format: &FileFormat {
        id: 3_836_321_158,
        source_type: SourceType::Iana,
        name: "vnd.marlin.drm.mdcf",
        extensions: &[],
        media_types: &["application/vnd.marlin.drm.mdcf"],
        signatures: &[],
        related_formats: &[],
    },
};
