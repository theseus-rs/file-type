use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_154740816: FileFormat = FileFormat {
    id: 154_740_816,
    source_type: SourceType::Iana,
    name: "vnd.geogebra.slides",
    extensions: &[],
    media_types: &["application/vnd.geogebra.slides"],
    internal_signatures: &[],
    related_formats: &[],
};
