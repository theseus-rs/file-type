use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2288013072: FileType = FileType {
    file_format: &FileFormat {
        id: 2_288_013_072,
        source_type: SourceType::Iana,
        name: "http",
        extensions: &[],
        media_types: &["application/http"],
        signatures: &[],
        related_formats: &[],
    },
};
