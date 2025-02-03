use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1135062890: FileFormat = FileFormat {
    id: 1_135_062_890,
    source_type: SourceType::Httpd,
    name: "bzip2",
    extensions: &["bz2", "boz"],
    media_types: &["application/x-bzip2"],
    internal_signatures: &[],
    related_formats: &[],
};
