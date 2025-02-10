use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3752320099: FileType = FileType {
    file_format: &FileFormat {
        id: 3_752_320_099,
        source_type: SourceType::Iana,
        name: "vnd.dvb.notif-ia-registration-request+xml",
        extensions: &[],
        media_types: &["application/vnd.dvb.notif-ia-registration-request+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
