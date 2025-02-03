use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3716723091890390689: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "xliff xml",
    extensions: &["xlf"],
    media_types: &["application/x-xliff+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
