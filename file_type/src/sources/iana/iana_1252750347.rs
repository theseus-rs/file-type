use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1252750347: FileType = FileType {
    file_format: &FileFormat {
        id: 1_252_750_347,
        source_type: SourceType::Iana,
        name: "fwdred",
        extensions: &[],
        media_types: &["text/fwdred"],
        signatures: &[],
        related_formats: &[],
    },
};
