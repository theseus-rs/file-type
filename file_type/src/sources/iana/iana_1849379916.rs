use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1849379916: FileType = FileType {
    file_format: &FileFormat {
        id: 1_849_379_916,
        source_type: SourceType::Iana,
        name: "x-www-form-urlencoded",
        extensions: &[],
        media_types: &["application/x-www-form-urlencoded"],
        signatures: &[],
        related_formats: &[],
    },
};
