use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1347231439: FileType = FileType {
    file_format: &FileFormat {
        id: 1_347_231_439,
        source_type: SourceType::Iana,
        name: "vnd.marlin.drm.license+xml",
        extensions: &[],
        media_types: &["application/vnd.marlin.drm.license+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
