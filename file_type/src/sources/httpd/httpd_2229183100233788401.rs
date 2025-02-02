use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2229183100233788401: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "igloader",
    extensions: &["igl"],
    media_types: &["application/vnd.igloader"],
    internal_signatures: &[],
    related_formats: &[],
};
