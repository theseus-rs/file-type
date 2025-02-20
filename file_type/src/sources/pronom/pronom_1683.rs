use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_1683: FileType = FileType {
    file_format: &FileFormat {
        id: 1_683,
        source_type: SourceType::Pronom,
        name: "Fortran",
        extensions: &["f90", "f95", "f03", "f", "for"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
