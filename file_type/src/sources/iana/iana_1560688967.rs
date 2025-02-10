use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1560688967: FileType = FileType {
    file_format: &FileFormat {
        id: 1_560_688_967,
        source_type: SourceType::Iana,
        name: "vnd.japannet-verification-wakeup",
        extensions: &[],
        media_types: &["application/vnd.japannet-verification-wakeup"],
        signatures: &[],
        related_formats: &[],
    },
};
