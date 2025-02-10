use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1049248689: FileType = FileType {
    file_format: &FileFormat {
        id: 1_049_248_689,
        source_type: SourceType::Iana,
        name: "vnd.dece.hd",
        extensions: &[],
        media_types: &["video/vnd.dece.hd"],
        signatures: &[],
        related_formats: &[],
    },
};
