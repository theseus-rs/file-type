use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2222656293: FileType = FileType {
    file_format: &FileFormat {
        id: 2_222_656_293,
        source_type: SourceType::Iana,
        name: "application/trust-mark+jwt",
        extensions: &[],
        media_types: &["application/trust-mark+jwt"],
        signatures: &[],
        related_formats: &[],
    },
};
