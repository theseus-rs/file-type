use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2735829481: FileType = FileType {
    file_format: &FileFormat {
        id: 2_735_829_481,
        source_type: SourceType::Iana,
        name: "quicktime",
        extensions: &[],
        media_types: &["video/quicktime"],
        signatures: &[],
        related_formats: &[],
    },
};
