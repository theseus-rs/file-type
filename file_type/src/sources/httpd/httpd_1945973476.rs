use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1945973476: FileType = FileType {
    file_format: &FileFormat {
        id: 1_945_973_476,
        source_type: SourceType::Httpd,
        name: "cif",
        extensions: &["cif"],
        media_types: &["chemical/x-cif"],
        signatures: &[],
        related_formats: &[],
    },
};
