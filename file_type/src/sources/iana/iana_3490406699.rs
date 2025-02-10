use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3490406699: FileType = FileType {
    file_format: &FileFormat {
        id: 3_490_406_699,
        source_type: SourceType::Iana,
        name: "signed",
        extensions: &[],
        media_types: &["multipart/signed"],
        signatures: &[],
        related_formats: &[],
    },
};
