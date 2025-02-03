use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3755495095313444404: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "yamaha hv script",
    extensions: &["hvs"],
    media_types: &["application/vnd.yamaha.hv-script"],
    internal_signatures: &[],
    related_formats: &[],
};
