use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2167490047: FileType = FileType {
    file_format: &FileFormat {
        id: 2_167_490_047,
        source_type: SourceType::Iana,
        name: "apng",
        extensions: &[],
        media_types: &["image/apng"],
        signatures: &[],
        related_formats: &[],
    },
};
