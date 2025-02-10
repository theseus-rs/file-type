use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3232900560: FileType = FileType {
    file_format: &FileFormat {
        id: 3_232_900_560,
        source_type: SourceType::Iana,
        name: "TETRA_ISI",
        extensions: &[],
        media_types: &["application/TETRA_ISI"],
        signatures: &[],
        related_formats: &[],
    },
};
