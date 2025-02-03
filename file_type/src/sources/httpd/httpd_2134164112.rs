use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2134164112: FileFormat = FileFormat {
    id: 2_134_164_112,
    source_type: SourceType::Httpd,
    name: "ms application",
    extensions: &["application"],
    media_types: &["application/x-ms-application"],
    internal_signatures: &[],
    related_formats: &[],
};
