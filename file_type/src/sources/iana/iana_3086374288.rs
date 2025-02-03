use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3086374288: FileFormat = FileFormat {
    id: 3_086_374_288,
    source_type: SourceType::Iana,
    name: "vnd.dece.graphic",
    extensions: &[],
    media_types: &["image/vnd.dece.graphic"],
    internal_signatures: &[],
    related_formats: &[],
};
