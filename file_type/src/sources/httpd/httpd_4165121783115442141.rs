use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4165121783115442141: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "airzip filesecure azf",
    extensions: &["azf"],
    media_types: &["application/vnd.airzip.filesecure.azf"],
    internal_signatures: &[],
    related_formats: &[],
};
