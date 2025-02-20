use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3864329248: FileType = FileType {
    file_format: &FileFormat {
        id: 3_864_329_248,
        source_type: SourceType::Iana,
        name: "vnd.debian.copyright",
        extensions: &[],
        media_types: &["text/vnd.debian.copyright"],
        signatures: &[],
        related_formats: &[],
    },
};
