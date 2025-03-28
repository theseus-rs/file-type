use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_68397272: FileType = FileType {
    file_format: &FileFormat {
        id: 68_397_272,
        source_type: SourceType::Iana,
        name: "vnd.fastbidsheet",
        extensions: &[],
        media_types: &["image/vnd.fastbidsheet"],
        signatures: &[],
        related_formats: &[],
    },
};
