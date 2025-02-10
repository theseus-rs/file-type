use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3320182908: FileType = FileType {
    file_format: &FileFormat {
        id: 3_320_182_908,
        source_type: SourceType::Iana,
        name: "vnd.oma.cab-subs-invite+xml",
        extensions: &[],
        media_types: &["application/vnd.oma.cab-subs-invite+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
