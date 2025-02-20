use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4052293624: FileType = FileType {
    file_format: &FileFormat {
        id: 4_052_293_624,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.seal-unicast-info+xml",
        extensions: &[],
        media_types: &["application/vnd.3gpp.seal-unicast-info+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
