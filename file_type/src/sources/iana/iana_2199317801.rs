use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2199317801: FileType = FileType {
    file_format: &FileFormat {
        id: 2_199_317_801,
        source_type: SourceType::Iana,
        name: "QSIG",
        extensions: &[],
        media_types: &["application/QSIG"],
        signatures: &[],
        related_formats: &[],
    },
};
