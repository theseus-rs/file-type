use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3811794466: FileType = FileType {
    file_format: &FileFormat {
        id: 3_811_794_466,
        source_type: SourceType::Iana,
        name: "vnd.hp-PCL",
        extensions: &[],
        media_types: &["application/vnd.hp-PCL"],
        signatures: &[],
        related_formats: &[],
    },
};
