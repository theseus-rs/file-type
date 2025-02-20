use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4082873929: FileType = FileType {
    file_format: &FileFormat {
        id: 4_082_873_929,
        source_type: SourceType::Httpd,
        name: "llamagraphics life balance exchange xml",
        extensions: &["lbe"],
        media_types: &["application/vnd.llamagraphics.life-balance.exchange+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
