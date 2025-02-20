use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2567268996: FileType = FileType {
    file_format: &FileFormat {
        id: 2_567_268_996,
        source_type: SourceType::Iana,
        name: "vnd.iptc.g2.conceptitem+xml",
        extensions: &[],
        media_types: &["application/vnd.iptc.g2.conceptitem+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
