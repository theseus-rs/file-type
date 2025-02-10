use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3748108668: FileType = FileType {
    file_format: &FileFormat {
        id: 3_748_108_668,
        source_type: SourceType::Iana,
        name: "vnd.sealedmedia.softseal.pdf",
        extensions: &[],
        media_types: &["application/vnd.sealedmedia.softseal.pdf"],
        signatures: &[],
        related_formats: &[],
    },
};
