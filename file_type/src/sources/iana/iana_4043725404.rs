use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4043725404: FileType = FileType {
    file_format: &FileFormat {
        id: 4_043_725_404,
        source_type: SourceType::Iana,
        name: "3gpp",
        extensions: &[],
        media_types: &["audio/3gpp"],
        signatures: &[],
        related_formats: &[],
    },
};
