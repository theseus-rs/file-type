use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3334640110: FileType = FileType {
    file_format: &FileFormat {
        id: 3_334_640_110,
        source_type: SourceType::Iana,
        name: "vnd.chemdraw+xml",
        extensions: &[],
        media_types: &["application/vnd.chemdraw+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
