use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_200188636: FileType = FileType {
    file_format: &FileFormat {
        id: 200_188_636,
        source_type: SourceType::Iana,
        name: "vnd.document+json",
        extensions: &[],
        media_types: &["application/vnd.document+json"],
        signatures: &[],
        related_formats: &[],
    },
};
