use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2461764934: FileFormat = FileFormat {
    id: 2_461_764_934,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.presentationml.slideLayout+xml",
    extensions: &[],
    media_types: &["application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml"],
    signatures: &[],
    related_formats: &[],
};
