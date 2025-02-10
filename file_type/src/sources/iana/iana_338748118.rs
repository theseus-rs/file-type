use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_338748118: FileType = FileType {
    file_format: &FileFormat {
        id: 338_748_118,
        source_type: SourceType::Iana,
        name: "vnd.informedcontrol.rms+xml",
        extensions: &[],
        media_types: &["application/vnd.informedcontrol.rms+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
