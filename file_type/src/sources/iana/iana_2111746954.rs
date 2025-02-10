use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2111746954: FileType = FileType {
    file_format: &FileFormat {
        id: 2_111_746_954,
        source_type: SourceType::Iana,
        name: "vnd.iptc.g2.newsmessage+xml",
        extensions: &[],
        media_types: &["application/vnd.iptc.g2.newsmessage+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
