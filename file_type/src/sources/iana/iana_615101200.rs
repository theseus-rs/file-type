use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_615101200: FileType = FileType {
    file_format: &FileFormat {
        id: 615_101_200,
        source_type: SourceType::Iana,
        name: "vnd.veryant.thin",
        extensions: &[],
        media_types: &["application/vnd.veryant.thin"],
        signatures: &[],
        related_formats: &[],
    },
};
