use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3435203577: FileType = FileType {
    file_format: &FileFormat {
        id: 3_435_203_577,
        source_type: SourceType::Iana,
        name: "eat+jwt",
        extensions: &[],
        media_types: &["application/eat+jwt"],
        signatures: &[],
        related_formats: &[],
    },
};
