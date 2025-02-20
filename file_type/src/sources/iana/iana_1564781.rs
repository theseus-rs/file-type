use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1564781: FileType = FileType {
    file_format: &FileFormat {
        id: 1_564_781,
        source_type: SourceType::Iana,
        name: "flexfec",
        extensions: &[],
        media_types: &["application/flexfec"],
        signatures: &[],
        related_formats: &[],
    },
};
