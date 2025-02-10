use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2282658075: FileType = FileType {
    file_format: &FileFormat {
        id: 2_282_658_075,
        source_type: SourceType::Iana,
        name: "vnd.dvb.notif-ia-msglist+xml",
        extensions: &[],
        media_types: &["application/vnd.dvb.notif-ia-msglist+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
