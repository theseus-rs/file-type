use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
