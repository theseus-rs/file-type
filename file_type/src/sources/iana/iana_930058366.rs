use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_930058366: FileType = FileType {
    file_format: &FileFormat {
        id: 930_058_366,
        source_type: SourceType::Iana,
        name: "vnd.in3d.spot",
        extensions: &[],
        media_types: &["text/vnd.in3d.spot"],
        signatures: &[],
        related_formats: &[],
    },
};
