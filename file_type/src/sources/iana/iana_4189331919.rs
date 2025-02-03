use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4189331919: FileFormat = FileFormat {
    id: 4_189_331_919,
    source_type: SourceType::Iana,
    name: "vnd.resilient.logic",
    extensions: &[],
    media_types: &["application/vnd.resilient.logic"],
    internal_signatures: &[],
    related_formats: &[],
};
