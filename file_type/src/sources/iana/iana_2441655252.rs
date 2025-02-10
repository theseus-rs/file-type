use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2441655252: FileType = FileType {
    file_format: &FileFormat {
        id: 2_441_655_252,
        source_type: SourceType::Iana,
        name: "vnd.oma.drm.risd+xml",
        extensions: &[],
        media_types: &["application/vnd.oma.drm.risd+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
