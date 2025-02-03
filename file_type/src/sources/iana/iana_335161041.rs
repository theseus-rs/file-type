use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_335161041: FileFormat = FileFormat {
    id: 335_161_041,
    source_type: SourceType::Iana,
    name: "vnd.hydrostatix.sof-data",
    extensions: &[],
    media_types: &["application/vnd.hydrostatix.sof-data"],
    internal_signatures: &[],
    related_formats: &[],
};
