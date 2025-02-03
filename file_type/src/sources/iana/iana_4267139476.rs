use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4267139476: FileFormat = FileFormat {
    id: 4_267_139_476,
    source_type: SourceType::Iana,
    name: "vnd.route66.link66+xml",
    extensions: &[],
    media_types: &["application/vnd.route66.link66+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
