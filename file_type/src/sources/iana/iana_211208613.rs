use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_211208613: FileType = FileType {
    file_format: &FileFormat {
        id: 211_208_613,
        source_type: SourceType::Iana,
        name: "xcon-conference-info+xml",
        extensions: &[],
        media_types: &["application/xcon-conference-info+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
