use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3386887635: FileType = FileType {
    file_format: &FileFormat {
        id: 3_386_887_635,
        source_type: SourceType::Iana,
        name: "vnd.obn",
        extensions: &[],
        media_types: &["application/vnd.obn"],
        signatures: &[],
        related_formats: &[],
    },
};
