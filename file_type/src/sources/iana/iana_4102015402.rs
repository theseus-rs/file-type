use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4102015402: FileType = FileType {
    file_format: &FileFormat {
        id: 4_102_015_402,
        source_type: SourceType::Iana,
        name: "rpki-ccr+gzip",
        extensions: &[],
        media_types: &["application/rpki-ccr+gzip"],
        signatures: &[],
        related_formats: &[],
    },
};
