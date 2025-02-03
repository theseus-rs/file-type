use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_272313846: FileFormat = FileFormat {
    id: 272_313_846,
    source_type: SourceType::Httpd,
    name: "ms artgalry",
    extensions: &["cil"],
    media_types: &["application/vnd.ms-artgalry"],
    internal_signatures: &[],
    related_formats: &[],
};
