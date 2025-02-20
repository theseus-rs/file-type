use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_565406949: FileType = FileType {
    file_format: &FileFormat {
        id: 565_406_949,
        source_type: SourceType::Iana,
        name: "vnd.intergeo",
        extensions: &[],
        media_types: &["application/vnd.intergeo"],
        signatures: &[],
        related_formats: &[],
    },
};
