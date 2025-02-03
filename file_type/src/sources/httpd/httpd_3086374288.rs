use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3086374288: FileFormat = FileFormat {
    id: 3_086_374_288,
    source_type: SourceType::Httpd,
    name: "dece graphic",
    extensions: &["uvi", "uvvi", "uvg", "uvvg"],
    media_types: &["image/vnd.dece.graphic"],
    internal_signatures: &[],
    related_formats: &[],
};
