use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3047948639: FileFormat = FileFormat {
    id: 3_047_948_639,
    source_type: SourceType::Iana,
    name: "vnd.bmi",
    extensions: &[],
    media_types: &["application/vnd.bmi"],
    internal_signatures: &[],
    related_formats: &[],
};
