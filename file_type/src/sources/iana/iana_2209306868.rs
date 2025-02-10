use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2209306868: FileType = FileType {
    file_format: &FileFormat {
        id: 2_209_306_868,
        source_type: SourceType::Iana,
        name: "rtp-enc-aescm128",
        extensions: &[],
        media_types: &["text/rtp-enc-aescm128"],
        signatures: &[],
        related_formats: &[],
    },
};
