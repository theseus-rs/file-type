use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_913650795: FileType = FileType {
    file_format: &FileFormat {
        id: 913_650_795,
        source_type: SourceType::Iana,
        name: "vnd.relpipe",
        extensions: &[],
        media_types: &["application/vnd.relpipe"],
        signatures: &[],
        related_formats: &[],
    },
};
