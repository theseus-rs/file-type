use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1936061579: FileType = FileType {
    file_format: &FileFormat {
        id: 1_936_061_579,
        source_type: SourceType::Iana,
        name: "vnd.marlin.drm.actiontoken+xml",
        extensions: &[],
        media_types: &["application/vnd.marlin.drm.actiontoken+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
