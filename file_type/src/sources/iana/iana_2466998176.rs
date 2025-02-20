use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2466998176: FileType = FileType {
    file_format: &FileFormat {
        id: 2_466_998_176,
        source_type: SourceType::Iana,
        name: "jxss",
        extensions: &[],
        media_types: &["image/jxss"],
        signatures: &[],
        related_formats: &[],
    },
};
