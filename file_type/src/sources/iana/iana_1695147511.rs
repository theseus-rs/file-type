use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1695147511: FileType = FileType {
    file_format: &FileFormat {
        id: 1_695_147_511,
        source_type: SourceType::Iana,
        name: "vnd.CELP",
        extensions: &[],
        media_types: &["audio/vnd.CELP"],
        signatures: &[],
        related_formats: &[],
    },
};
