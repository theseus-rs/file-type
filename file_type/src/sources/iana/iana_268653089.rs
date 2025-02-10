use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_268653089: FileType = FileType {
    file_format: &FileFormat {
        id: 268_653_089,
        source_type: SourceType::Iana,
        name: "vnd.oasis.opendocument.text-web",
        extensions: &[],
        media_types: &["application/vnd.oasis.opendocument.text-web"],
        signatures: &[],
        related_formats: &[],
    },
};
