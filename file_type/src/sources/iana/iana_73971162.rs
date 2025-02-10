use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_73971162: FileType = FileType {
    file_format: &FileFormat {
        id: 73_971_162,
        source_type: SourceType::Iana,
        name: "provenance-notation",
        extensions: &[],
        media_types: &["text/provenance-notation"],
        signatures: &[],
        related_formats: &[],
    },
};
