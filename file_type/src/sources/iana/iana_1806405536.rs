use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1806405536: FileType = FileType {
    file_format: &FileFormat {
        id: 1_806_405_536,
        source_type: SourceType::Iana,
        name: "vnd.etsi.iptvdiscovery+xml",
        extensions: &[],
        media_types: &["application/vnd.etsi.iptvdiscovery+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
