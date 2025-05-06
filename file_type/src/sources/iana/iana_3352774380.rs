use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3352774380: FileType = FileType {
    file_format: &FileFormat {
        id: 3_352_774_380,
        source_type: SourceType::Iana,
        name: "vnd.wantverse",
        extensions: &[],
        media_types: &["application/vnd.wantverse"],
        signatures: &[],
        related_formats: &[],
    },
};
