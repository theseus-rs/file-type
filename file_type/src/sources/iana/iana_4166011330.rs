use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4166011330: FileType = FileType {
    file_format: &FileFormat {
        id: 4_166_011_330,
        source_type: SourceType::Iana,
        name: "rtp-enc-aescm128",
        extensions: &[],
        media_types: &["audio/rtp-enc-aescm128"],
        signatures: &[],
        related_formats: &[],
    },
};
