use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2762449500: FileType = FileType {
    file_format: &FileFormat {
        id: 2_762_449_500,
        source_type: SourceType::Iana,
        name: "vnd.iptc.g2.newsitem+xml",
        extensions: &[],
        media_types: &["application/vnd.iptc.g2.newsitem+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
