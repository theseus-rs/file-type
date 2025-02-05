use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_511568138: FileFormat = FileFormat {
    id: 511_568_138,
    source_type: SourceType::Httpd,
    name: "geogebra file",
    extensions: &["ggb"],
    media_types: &["application/vnd.geogebra.file"],
    signatures: &[],
    related_formats: &[],
};
