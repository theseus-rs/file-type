use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_667822579: FileType = FileType {
    file_format: &FileFormat {
        id: 667_822_579,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.seal-mbs-usage-info+xml",
        extensions: &[],
        media_types: &["application/vnd.3gpp.seal-mbs-usage-info+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
