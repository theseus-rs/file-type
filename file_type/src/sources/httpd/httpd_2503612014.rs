use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2503612014: FileFormat = FileFormat {
    id: 2_503_612_014,
    source_type: SourceType::Httpd,
    name: "mobius msl",
    extensions: &["msl"],
    media_types: &["application/vnd.mobius.msl"],
    signatures: &[],
    related_formats: &[],
};
