use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2978665677: FileFormat = FileFormat {
    id: 2_978_665_677,
    source_type: SourceType::Httpd,
    name: "claymore",
    extensions: &["cla"],
    media_types: &["application/vnd.claymore"],
    signatures: &[],
    related_formats: &[],
};
