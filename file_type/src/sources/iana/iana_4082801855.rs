use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4082801855: FileType = FileType {
    file_format: &FileFormat {
        id: 4_082_801_855,
        source_type: SourceType::Iana,
        name: "vnd.dvb.notif-container+xml",
        extensions: &[],
        media_types: &["application/vnd.dvb.notif-container+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
