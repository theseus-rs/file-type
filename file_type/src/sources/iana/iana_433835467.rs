use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_433835467: FileType = FileType {
    file_format: &FileFormat {
        id: 433_835_467,
        source_type: SourceType::Iana,
        name: "news-groupinfo",
        extensions: &[],
        media_types: &["application/news-groupinfo"],
        signatures: &[],
        related_formats: &[],
    },
};
