use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4166067584: FileType = FileType {
    file_format: &FileFormat {
        id: 4_166_067_584,
        source_type: SourceType::Iana,
        name: "mediaservercontrol+xml",
        extensions: &[],
        media_types: &["application/mediaservercontrol+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
