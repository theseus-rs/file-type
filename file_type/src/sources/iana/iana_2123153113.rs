use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2123153113: FileFormat = FileFormat {
    id: 2_123_153_113,
    source_type: SourceType::Iana,
    name: "vnd.ecowin.filerequest",
    extensions: &[],
    media_types: &["application/vnd.ecowin.filerequest"],
    internal_signatures: &[],
    related_formats: &[],
};
