use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2563513024: FileType = FileType {
    file_format: &FileFormat {
        id: 2_563_513_024,
        source_type: SourceType::Iana,
        name: "p21+zip",
        extensions: &[],
        media_types: &["application/p21+zip"],
        signatures: &[],
        related_formats: &[],
    },
};
