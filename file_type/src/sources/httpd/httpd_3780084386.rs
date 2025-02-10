use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3780084386: FileType = FileType {
    file_format: &FileFormat {
        id: 3_780_084_386,
        source_type: SourceType::Httpd,
        name: "zzazz deck xml",
        extensions: &["zaz"],
        media_types: &["application/vnd.zzazz.deck+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
