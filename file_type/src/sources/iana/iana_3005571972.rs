use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3005571972: FileType = FileType {
    file_format: &FileFormat {
        id: 3_005_571_972,
        source_type: SourceType::Iana,
        name: "vnd.byu.uapi+json",
        extensions: &[],
        media_types: &["application/vnd.byu.uapi+json"],
        signatures: &[],
        related_formats: &[],
    },
};
