use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1407868785: FileType = FileType {
    file_format: &FileFormat {
        id: 1_407_868_785,
        source_type: SourceType::Iana,
        name: "vnd.age",
        extensions: &[],
        media_types: &["application/vnd.age"],
        signatures: &[],
        related_formats: &[],
    },
};
