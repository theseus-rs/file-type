use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_266187991: FileType = FileType {
    file_format: &FileFormat {
        id: 266_187_991,
        source_type: SourceType::Iana,
        name: "vnd.dvb.notif-ia-registration-response+xml",
        extensions: &[],
        media_types: &["application/vnd.dvb.notif-ia-registration-response+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
