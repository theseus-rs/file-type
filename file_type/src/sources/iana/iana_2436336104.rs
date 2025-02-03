use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2436336104: FileFormat = FileFormat {
    id: 2_436_336_104,
    source_type: SourceType::Iana,
    name: "vnd.onvif.metadata",
    extensions: &[],
    media_types: &["application/vnd.onvif.metadata"],
    internal_signatures: &[],
    related_formats: &[],
};
