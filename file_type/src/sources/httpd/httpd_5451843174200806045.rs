use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_5451843174200806045: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "geogebra tool",
    extensions: &["ggt"],
    media_types: &["application/vnd.geogebra.tool"],
    internal_signatures: &[],
    related_formats: &[],
};
