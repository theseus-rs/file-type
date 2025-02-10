use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1393458892: FileType = FileType {
    file_format: &FileFormat {
        id: 1_393_458_892,
        source_type: SourceType::Iana,
        name: "vnd.logipipe.circuit+zip",
        extensions: &[],
        media_types: &["application/vnd.logipipe.circuit+zip"],
        signatures: &[],
        related_formats: &[],
    },
};
