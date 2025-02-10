use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_845986989: FileType = FileType {
    file_format: &FileFormat {
        id: 845_986_989,
        source_type: SourceType::Iana,
        name: "vnd.dvb.notif-aggregate-root+xml",
        extensions: &[],
        media_types: &["application/vnd.dvb.notif-aggregate-root+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
