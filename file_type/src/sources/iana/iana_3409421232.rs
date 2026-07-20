use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3409421232: FileType = FileType {
    file_format: &FileFormat {
        id: 3_409_421_232,
        source_type: SourceType::Iana,
        name: "vnd.bovnar",
        extensions: &[],
        media_types: &["text/vnd.bovnar"],
        signatures: &[],
        related_formats: &[],
    },
};
