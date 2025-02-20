use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2764967758: FileType = FileType {
    file_format: &FileFormat {
        id: 2_764_967_758,
        source_type: SourceType::Iana,
        name: "scvp-cv-request",
        extensions: &[],
        media_types: &["application/scvp-cv-request"],
        signatures: &[],
        related_formats: &[],
    },
};
