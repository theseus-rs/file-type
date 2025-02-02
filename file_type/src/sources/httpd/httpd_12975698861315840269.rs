use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_12975698861315840269: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "geogebra slides",
    extensions: &["ggs"],
    media_types: &["application/vnd.geogebra.slides"],
    internal_signatures: &[],
    related_formats: &[],
};
