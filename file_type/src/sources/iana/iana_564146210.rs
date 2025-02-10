use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_564146210: FileType = FileType {
    file_format: &FileFormat {
        id: 564_146_210,
        source_type: SourceType::Iana,
        name: "macwriteii",
        extensions: &[],
        media_types: &["application/macwriteii"],
        signatures: &[],
        related_formats: &[],
    },
};
