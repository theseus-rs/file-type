use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_147689487279366576: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "pkcs10",
    extensions: &["p10"],
    media_types: &["application/pkcs10"],
    internal_signatures: &[],
    related_formats: &[],
};
