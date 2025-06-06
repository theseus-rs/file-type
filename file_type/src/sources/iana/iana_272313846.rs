use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_272313846: FileType = FileType {
    file_format: &FileFormat {
        id: 272_313_846,
        source_type: SourceType::Iana,
        name: "vnd.ms-artgalry",
        extensions: &[],
        media_types: &["application/vnd.ms-artgalry"],
        signatures: &[],
        related_formats: &[],
    },
};
