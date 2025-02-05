use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3000375204: FileFormat = FileFormat {
    id: 3_000_375_204,
    source_type: SourceType::Iana,
    name: "vnd.oma.poc.detailed-progress-report+xml",
    extensions: &[],
    media_types: &["application/vnd.oma.poc.detailed-progress-report+xml"],
    signatures: &[],
    related_formats: &[],
};
