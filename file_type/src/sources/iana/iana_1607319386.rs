use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1607319386: FileType = FileType {
    file_format: &FileFormat {
        id: 1_607_319_386,
        source_type: SourceType::Iana,
        name: "vp+cose",
        extensions: &[],
        media_types: &["application/vp+cose"],
        signatures: &[],
        related_formats: &[],
    },
};
