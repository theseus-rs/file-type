use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_383186215: FileFormat = FileFormat {
    id: 383_186_215,
    source_type: SourceType::Iana,
    name: "vc",
    extensions: &[],
    media_types: &["application/vc"],
    internal_signatures: &[],
    related_formats: &[],
};
