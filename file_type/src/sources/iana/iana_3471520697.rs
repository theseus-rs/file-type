use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3471520697: FileType = FileType {
    file_format: &FileFormat {
        id: 3_471_520_697,
        source_type: SourceType::Iana,
        name: "vnd.uplanet.channel-wbxml",
        extensions: &[],
        media_types: &["application/vnd.uplanet.channel-wbxml"],
        signatures: &[],
        related_formats: &[],
    },
};
