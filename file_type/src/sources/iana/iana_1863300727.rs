use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1863300727: FileType = FileType {
    file_format: &FileFormat {
        id: 1_863_300_727,
        source_type: SourceType::Iana,
        name: "ip-mr_v2.5",
        extensions: &[],
        media_types: &["audio/ip-mr_v2.5"],
        signatures: &[],
        related_formats: &[],
    },
};
