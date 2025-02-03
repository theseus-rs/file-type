use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_154740816: FileFormat = FileFormat {
    id: 154_740_816,
    source_type: SourceType::Httpd,
    name: "geogebra slides",
    extensions: &["ggs"],
    media_types: &["application/vnd.geogebra.slides"],
    internal_signatures: &[],
    related_formats: &[],
};
