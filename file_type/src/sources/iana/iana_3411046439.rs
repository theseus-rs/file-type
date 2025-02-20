use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3411046439: FileType = FileType {
    file_format: &FileFormat {
        id: 3_411_046_439,
        source_type: SourceType::Iana,
        name: "vnd.fujifilm.fb.jfi+xml",
        extensions: &[],
        media_types: &["application/vnd.fujifilm.fb.jfi+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
