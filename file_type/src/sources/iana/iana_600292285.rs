use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_600292285: FileType = FileType {
    file_format: &FileFormat {
        id: 600_292_285,
        source_type: SourceType::Iana,
        name: "prs.plucker",
        extensions: &[],
        media_types: &["application/prs.plucker"],
        signatures: &[],
        related_formats: &[],
    },
};
