use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2988840305: FileType = FileType {
    file_format: &FileFormat {
        id: 2_988_840_305,
        source_type: SourceType::Iana,
        name: "lottie+json",
        extensions: &[],
        media_types: &["video/lottie+json"],
        signatures: &[],
        related_formats: &[],
    },
};
