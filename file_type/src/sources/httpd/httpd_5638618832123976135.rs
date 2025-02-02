use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_5638618832123976135: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "xenc xml",
    extensions: &["xenc"],
    media_types: &["application/xenc+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
