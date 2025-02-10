use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1478743956: FileType = FileType {
    file_format: &FileFormat {
        id: 1_478_743_956,
        source_type: SourceType::Iana,
        name: "vnd.gentoo.gpkg",
        extensions: &[],
        media_types: &["application/vnd.gentoo.gpkg"],
        signatures: &[],
        related_formats: &[],
    },
};
