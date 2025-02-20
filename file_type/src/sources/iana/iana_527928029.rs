use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_527928029: FileType = FileType {
    file_format: &FileFormat {
        id: 527_928_029,
        source_type: SourceType::Iana,
        name: "java-archive",
        extensions: &[],
        media_types: &["application/java-archive"],
        signatures: &[],
        related_formats: &[],
    },
};
