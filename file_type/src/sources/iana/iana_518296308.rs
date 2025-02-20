use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_518296308: FileType = FileType {
    file_format: &FileFormat {
        id: 518_296_308,
        source_type: SourceType::Iana,
        name: "vnd.etsi.iptvsync+xml",
        extensions: &[],
        media_types: &["application/vnd.etsi.iptvsync+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
