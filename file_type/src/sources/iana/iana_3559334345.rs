use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3559334345: FileType = FileType {
    file_format: &FileFormat {
        id: 3_559_334_345,
        source_type: SourceType::Iana,
        name: "news-transmission",
        extensions: &[],
        media_types: &["application/news-transmission"],
        signatures: &[],
        related_formats: &[],
    },
};
