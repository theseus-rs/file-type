use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
