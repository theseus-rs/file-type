use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3537265699: FileFormat = FileFormat {
    id: 3_537_265_699,
    source_type: SourceType::Iana,
    name: "vnd.lotus-freelance",
    extensions: &[],
    media_types: &["application/vnd.lotus-freelance"],
    signatures: &[],
    related_formats: &[],
};
