use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3836671704: FileType = FileType {
    file_format: &FileFormat {
        id: 3_836_671_704,
        source_type: SourceType::Iana,
        name: "index.response",
        extensions: &[],
        media_types: &["application/index.response"],
        signatures: &[],
        related_formats: &[],
    },
};
