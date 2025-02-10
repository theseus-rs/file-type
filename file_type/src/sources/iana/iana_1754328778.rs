use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1754328778: FileType = FileType {
    file_format: &FileFormat {
        id: 1_754_328_778,
        source_type: SourceType::Iana,
        name: "vnd.japannet-jpnstore-wakeup",
        extensions: &[],
        media_types: &["application/vnd.japannet-jpnstore-wakeup"],
        signatures: &[],
        related_formats: &[],
    },
};
