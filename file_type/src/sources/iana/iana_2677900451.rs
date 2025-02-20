use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2677900451: FileType = FileType {
    file_format: &FileFormat {
        id: 2_677_900_451,
        source_type: SourceType::Iana,
        name: "vnd.acucorp",
        extensions: &[],
        media_types: &["application/vnd.acucorp"],
        signatures: &[],
        related_formats: &[],
    },
};
