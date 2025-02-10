use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_712815207: FileType = FileType {
    file_format: &FileFormat {
        id: 712_815_207,
        source_type: SourceType::Iana,
        name: "efi",
        extensions: &[],
        media_types: &["application/efi"],
        signatures: &[],
        related_formats: &[],
    },
};
