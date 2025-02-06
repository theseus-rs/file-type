use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4048314563: FileFormat = FileFormat {
    id: 4_048_314_563,
    source_type: SourceType::Iana,
    name: "vnd.oipf.dae.xhtml+xml",
    extensions: &[],
    media_types: &["application/vnd.oipf.dae.xhtml+xml"],
    signatures: &[],
    related_formats: &[],
};
