use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1822006688: FileType = FileType {
    file_format: &FileFormat {
        id: 1_822_006_688,
        source_type: SourceType::Iana,
        name: "raptorfec",
        extensions: &[],
        media_types: &["application/raptorfec"],
        signatures: &[],
        related_formats: &[],
    },
};
