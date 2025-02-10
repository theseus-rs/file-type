use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1419891179: FileType = FileType {
    file_format: &FileFormat {
        id: 1_419_891_179,
        source_type: SourceType::Iana,
        name: "vnd.IPTC.NewsML",
        extensions: &[],
        media_types: &["text/vnd.IPTC.NewsML"],
        signatures: &[],
        related_formats: &[],
    },
};
