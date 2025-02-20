use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
