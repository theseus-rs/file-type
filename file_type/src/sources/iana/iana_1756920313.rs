use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1756920313: FileType = FileType {
    file_format: &FileFormat {
        id: 1_756_920_313,
        source_type: SourceType::Iana,
        name: "encaprtp",
        extensions: &[],
        media_types: &["video/encaprtp"],
        signatures: &[],
        related_formats: &[],
    },
};
