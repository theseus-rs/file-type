use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3336330106: FileType = FileType {
    file_format: &FileFormat {
        id: 3_336_330_106,
        source_type: SourceType::Iana,
        name: "vnd.dvb.ipdcdftnotifaccess",
        extensions: &[],
        media_types: &["application/vnd.dvb.ipdcdftnotifaccess"],
        signatures: &[],
        related_formats: &[],
    },
};
