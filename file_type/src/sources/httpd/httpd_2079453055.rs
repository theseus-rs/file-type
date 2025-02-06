use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2079453055: FileFormat = FileFormat {
    id: 2_079_453_055,
    source_type: SourceType::Httpd,
    name: "tex tfm",
    extensions: &["tfm"],
    media_types: &["application/x-tex-tfm"],
    signatures: &[],
    related_formats: &[],
};
