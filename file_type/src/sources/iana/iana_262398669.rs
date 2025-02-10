use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_262398669: FileType = FileType {
    file_format: &FileFormat {
        id: 262_398_669,
        source_type: SourceType::Iana,
        name: "vnd.dvb.dvbisl+xml",
        extensions: &[],
        media_types: &["application/vnd.dvb.dvbisl+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
