use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_868618244: FileType = FileType {
    file_format: &FileFormat {
        id: 868_618_244,
        source_type: SourceType::Iana,
        name: "vnd.ahead.space",
        extensions: &[],
        media_types: &["application/vnd.ahead.space"],
        signatures: &[],
        related_formats: &[],
    },
};
