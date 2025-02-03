use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4293215786: FileFormat = FileFormat {
    id: 4_293_215_786,
    source_type: SourceType::Httpd,
    name: "acucobol",
    extensions: &["acu"],
    media_types: &["application/vnd.acucobol"],
    internal_signatures: &[],
    related_formats: &[],
};
