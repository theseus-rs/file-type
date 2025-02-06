use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1770716747: FileFormat = FileFormat {
    id: 1_770_716_747,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.mcvideo-location-info+xml",
    extensions: &[],
    media_types: &["application/vnd.3gpp.mcvideo-location-info+xml"],
    signatures: &[],
    related_formats: &[],
};
