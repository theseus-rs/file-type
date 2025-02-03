use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2411103515: FileFormat = FileFormat {
    id: 2_411_103_515,
    source_type: SourceType::Iana,
    name: "vnd.syft+json",
    extensions: &[],
    media_types: &["application/vnd.syft+json"],
    internal_signatures: &[],
    related_formats: &[],
};
