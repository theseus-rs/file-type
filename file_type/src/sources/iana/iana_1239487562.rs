use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1239487562: FileType = FileType {
    file_format: &FileFormat {
        id: 1_239_487_562,
        source_type: SourceType::Iana,
        name: "MELP1200",
        extensions: &[],
        media_types: &["audio/MELP1200"],
        signatures: &[],
        related_formats: &[],
    },
};
