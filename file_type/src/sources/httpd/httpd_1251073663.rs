use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1251073663: FileFormat = FileFormat {
    id: 1_251_073_663,
    source_type: SourceType::Httpd,
    name: "nitf",
    extensions: &["ntf", "nitf"],
    media_types: &["application/vnd.nitf"],
    internal_signatures: &[],
    related_formats: &[],
};
