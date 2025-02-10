use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3063913671: FileType = FileType {
    file_format: &FileFormat {
        id: 3_063_913_671,
        source_type: SourceType::Iana,
        name: "vnd.imagemeter.image+zip",
        extensions: &[],
        media_types: &["application/vnd.imagemeter.image+zip"],
        signatures: &[],
        related_formats: &[],
    },
};
