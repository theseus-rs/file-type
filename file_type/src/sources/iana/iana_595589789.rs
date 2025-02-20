use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_595589789: FileType = FileType {
    file_format: &FileFormat {
        id: 595_589_789,
        source_type: SourceType::Iana,
        name: "vnd.etsi.iptvcommand+xml",
        extensions: &[],
        media_types: &["application/vnd.etsi.iptvcommand+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
