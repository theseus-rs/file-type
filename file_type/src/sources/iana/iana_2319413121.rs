use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2319413121: FileType = FileType {
    file_format: &FileFormat {
        id: 2_319_413_121,
        source_type: SourceType::Iana,
        name: "parityfec",
        extensions: &[],
        media_types: &["text/parityfec"],
        signatures: &[],
        related_formats: &[],
    },
};
