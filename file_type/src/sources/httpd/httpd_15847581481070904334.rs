use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_15847581481070904334: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "bzip2",
    extensions: &["bz2", "boz"],
    media_types: &["application/x-bzip2"],
    internal_signatures: &[],
    related_formats: &[],
};
