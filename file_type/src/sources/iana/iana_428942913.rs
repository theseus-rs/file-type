use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_428942913: FileType = FileType {
    file_format: &FileFormat {
        id: 428_942_913,
        source_type: SourceType::Iana,
        name: "vnd.agtp.identity+yaml",
        extensions: &[],
        media_types: &["application/vnd.agtp.identity+yaml"],
        signatures: &[],
        related_formats: &[],
    },
};
