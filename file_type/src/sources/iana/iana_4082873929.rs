use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4082873929: FileType = FileType {
    file_format: &FileFormat {
        id: 4_082_873_929,
        source_type: SourceType::Iana,
        name: "vnd.llamagraphics.life-balance.exchange+xml",
        extensions: &[],
        media_types: &["application/vnd.llamagraphics.life-balance.exchange+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
