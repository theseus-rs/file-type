use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_761352333: FileType = FileType {
    file_format: &FileFormat {
        id: 761_352_333,
        source_type: SourceType::Linguist,
        name: "Fortran Free Form",
        extensions: &["f03", "f08", "f90", "f95"],
        media_types: &["text/x-fortran"],
        signatures: &[],
        related_formats: &[],
    },
};
