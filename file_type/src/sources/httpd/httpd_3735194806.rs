use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3735194806: FileFormat = FileFormat {
    id: 3_735_194_806,
    source_type: SourceType::Httpd,
    name: "sema",
    extensions: &["sema"],
    media_types: &["application/vnd.sema"],
    internal_signatures: &[],
    related_formats: &[],
};
