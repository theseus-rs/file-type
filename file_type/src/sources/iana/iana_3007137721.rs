use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3007137721: FileFormat = FileFormat {
    id: 3_007_137_721,
    source_type: SourceType::Iana,
    name: "mbms-reception-report+xml",
    extensions: &[],
    media_types: &["application/mbms-reception-report+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
