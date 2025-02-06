use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1478743956: FileFormat = FileFormat {
    id: 1_478_743_956,
    source_type: SourceType::Iana,
    name: "vnd.gentoo.gpkg",
    extensions: &[],
    media_types: &["application/vnd.gentoo.gpkg"],
    signatures: &[],
    related_formats: &[],
};
