use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1190679210: FileType = FileType {
    file_format: &FileFormat {
        id: 1_190_679_210,
        source_type: SourceType::Iana,
        name: "vnd.supercard-pro-disk-image",
        extensions: &[],
        media_types: &["application/vnd.supercard-pro-disk-image"],
        signatures: &[],
        related_formats: &[],
    },
};
