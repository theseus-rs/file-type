use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3908223514: FileType = FileType {
    file_format: &FileFormat {
        id: 3_908_223_514,
        source_type: SourceType::Iana,
        name: "vnd.sealed.png",
        extensions: &[],
        media_types: &["image/vnd.sealed.png"],
        signatures: &[],
        related_formats: &[],
    },
};
