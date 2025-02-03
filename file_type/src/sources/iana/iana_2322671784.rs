use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2322671784: FileFormat = FileFormat {
    id: 2_322_671_784,
    source_type: SourceType::Iana,
    name: "vnd.oma.poc.optimized-progress-report+xml",
    extensions: &[],
    media_types: &["application/vnd.oma.poc.optimized-progress-report+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
