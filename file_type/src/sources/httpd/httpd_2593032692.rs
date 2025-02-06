use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2593032692: FileFormat = FileFormat {
    id: 2_593_032_692,
    source_type: SourceType::Httpd,
    name: "sun xml calc template",
    extensions: &["stc"],
    media_types: &["application/vnd.sun.xml.calc.template"],
    signatures: &[],
    related_formats: &[],
};
