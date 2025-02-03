use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3810885982: FileFormat = FileFormat {
    id: 3_810_885_982,
    source_type: SourceType::Iana,
    name: "linkset+json",
    extensions: &[],
    media_types: &["application/linkset+json"],
    internal_signatures: &[],
    related_formats: &[],
};
