use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2539465323: FileFormat = FileFormat {
    id: 2_539_465_323,
    source_type: SourceType::Iana,
    name: "png",
    extensions: &[],
    media_types: &["image/png"],
    signatures: &[],
    related_formats: &[],
};
