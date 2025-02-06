use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1794933711: FileFormat = FileFormat {
    id: 1_794_933_711,
    source_type: SourceType::Iana,
    name: "jcr-cnd",
    extensions: &[],
    media_types: &["text/jcr-cnd"],
    signatures: &[],
    related_formats: &[],
};
