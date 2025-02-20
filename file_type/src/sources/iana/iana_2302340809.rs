use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2302340809: FileType = FileType {
    file_format: &FileFormat {
        id: 2_302_340_809,
        source_type: SourceType::Iana,
        name: "fits",
        extensions: &[],
        media_types: &["image/fits"],
        signatures: &[],
        related_formats: &[],
    },
};
