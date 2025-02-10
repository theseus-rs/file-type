use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4266951372: FileType = FileType {
    file_format: &FileFormat {
        id: 4_266_951_372,
        source_type: SourceType::Iana,
        name: "vnd.nokia.ncd",
        extensions: &[],
        media_types: &["application/vnd.nokia.ncd"],
        signatures: &[],
        related_formats: &[],
    },
};
