use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1877781678: FileType = FileType {
    file_format: &FileFormat {
        id: 1_877_781_678,
        source_type: SourceType::Iana,
        name: "vnd.oma.xcap-directory+xml",
        extensions: &[],
        media_types: &["application/vnd.oma.xcap-directory+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
