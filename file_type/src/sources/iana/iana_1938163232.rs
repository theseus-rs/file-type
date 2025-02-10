use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1938163232: FileType = FileType {
    file_format: &FileFormat {
        id: 1_938_163_232,
        source_type: SourceType::Iana,
        name: "vnd.zul",
        extensions: &[],
        media_types: &["application/vnd.zul"],
        signatures: &[],
        related_formats: &[],
    },
};
