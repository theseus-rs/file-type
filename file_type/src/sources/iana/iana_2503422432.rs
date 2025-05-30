use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2503422432: FileType = FileType {
    file_format: &FileFormat {
        id: 2_503_422_432,
        source_type: SourceType::Iana,
        name: "vnd.sketchometry",
        extensions: &[],
        media_types: &["application/vnd.sketchometry"],
        signatures: &[],
        related_formats: &[],
    },
};
