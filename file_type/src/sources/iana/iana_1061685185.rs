use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1061685185: FileType = FileType {
    file_format: &FileFormat {
        id: 1_061_685_185,
        source_type: SourceType::Iana,
        name: "vnd.svr.receipt+json",
        extensions: &[],
        media_types: &["application/vnd.svr.receipt+json"],
        signatures: &[],
        related_formats: &[],
    },
};
