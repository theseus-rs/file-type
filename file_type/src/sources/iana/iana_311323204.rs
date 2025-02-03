use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_311323204: FileFormat = FileFormat {
    id: 311_323_204,
    source_type: SourceType::Iana,
    name: "vnd.nokia.iptv.config+xml",
    extensions: &[],
    media_types: &["application/vnd.nokia.iptv.config+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
