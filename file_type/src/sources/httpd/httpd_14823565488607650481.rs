use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_14823565488607650481: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "mpeg",
    extensions: &["mpeg", "mpg", "mpe", "m1v", "m2v"],
    media_types: &["video/mpeg"],
    internal_signatures: &[],
    related_formats: &[],
};
