use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_214210857: FileType = FileType {
    file_format: &FileFormat {
        id: 214_210_857,
        source_type: SourceType::Iana,
        name: "cfw",
        extensions: &[],
        media_types: &["application/cfw"],
        signatures: &[],
        related_formats: &[],
    },
};
