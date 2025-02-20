use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2434472940: FileType = FileType {
    file_format: &FileFormat {
        id: 2_434_472_940,
        source_type: SourceType::Iana,
        name: "vnd.airzip.filesecure.azf",
        extensions: &[],
        media_types: &["application/vnd.airzip.filesecure.azf"],
        signatures: &[],
        related_formats: &[],
    },
};
