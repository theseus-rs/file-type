use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4052101019: FileType = FileType {
    file_format: &FileFormat {
        id: 4_052_101_019,
        source_type: SourceType::Iana,
        name: "news-checkgroups",
        extensions: &[],
        media_types: &["application/news-checkgroups"],
        signatures: &[],
        related_formats: &[],
    },
};
