use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3588544249: FileType = FileType {
    file_format: &FileFormat {
        id: 3_588_544_249,
        source_type: SourceType::Iana,
        name: "express",
        extensions: &[],
        media_types: &["application/express"],
        signatures: &[],
        related_formats: &[],
    },
};
