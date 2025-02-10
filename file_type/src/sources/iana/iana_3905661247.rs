use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3905661247: FileType = FileType {
    file_format: &FileFormat {
        id: 3_905_661_247,
        source_type: SourceType::Iana,
        name: "vnd.oasis.opendocument.graphics",
        extensions: &[],
        media_types: &["application/vnd.oasis.opendocument.graphics"],
        signatures: &[],
        related_formats: &[],
    },
};
