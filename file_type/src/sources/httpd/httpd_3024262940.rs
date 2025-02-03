use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3024262940: FileFormat = FileFormat {
    id: 3_024_262_940,
    source_type: SourceType::Httpd,
    name: "geoplan",
    extensions: &["g2w"],
    media_types: &["application/vnd.geoplan"],
    internal_signatures: &[],
    related_formats: &[],
};
