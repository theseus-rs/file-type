use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_370260795: FileFormat = FileFormat {
    id: 370_260_795,
    source_type: SourceType::Iana,
    name: "vnd.oai.workflows",
    extensions: &[],
    media_types: &["application/vnd.oai.workflows"],
    internal_signatures: &[],
    related_formats: &[],
};
