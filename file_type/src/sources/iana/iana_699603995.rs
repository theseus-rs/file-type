use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_699603995: FileType = FileType {
    file_format: &FileFormat {
        id: 699_603_995,
        source_type: SourceType::Iana,
        name: "external-body",
        extensions: &[],
        media_types: &["message/external-body"],
        signatures: &[],
        related_formats: &[],
    },
};
