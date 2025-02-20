use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1407101828: FileType = FileType {
    file_format: &FileFormat {
        id: 1_407_101_828,
        source_type: SourceType::Iana,
        name: "vnd.dece.data",
        extensions: &[],
        media_types: &["application/vnd.dece.data"],
        signatures: &[],
        related_formats: &[],
    },
};
