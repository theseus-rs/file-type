use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3009405301600505228: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "3gpp",
    extensions: &["3gp"],
    media_types: &["video/3gpp"],
    internal_signatures: &[],
    related_formats: &[],
};
