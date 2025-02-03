use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_18438421708258998726: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "geoplan",
    extensions: &["g2w"],
    media_types: &["application/vnd.geoplan"],
    internal_signatures: &[],
    related_formats: &[],
};
