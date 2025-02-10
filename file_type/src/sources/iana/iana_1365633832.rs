use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1365633832: FileType = FileType {
    file_format: &FileFormat {
        id: 1_365_633_832,
        source_type: SourceType::Iana,
        name: "vnd.abc",
        extensions: &[],
        media_types: &["text/vnd.abc"],
        signatures: &[],
        related_formats: &[],
    },
};
