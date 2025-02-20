use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_512588624: FileType = FileType {
    file_format: &FileFormat {
        id: 512_588_624,
        source_type: SourceType::Iana,
        name: "vnd.iptc.g2.knowledgeitem+xml",
        extensions: &[],
        media_types: &["application/vnd.iptc.g2.knowledgeitem+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
