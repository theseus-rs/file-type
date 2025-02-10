use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4016775256: FileType = FileType {
    file_format: &FileFormat {
        id: 4_016_775_256,
        source_type: SourceType::Iana,
        name: "linkset",
        extensions: &[],
        media_types: &["application/linkset"],
        signatures: &[],
        related_formats: &[],
    },
};
