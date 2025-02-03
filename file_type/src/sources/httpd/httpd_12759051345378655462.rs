use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_12759051345378655462: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "freehand",
    extensions: &["fh", "fhc", "fh4", "fh5", "fh7"],
    media_types: &["image/x-freehand"],
    internal_signatures: &[],
    related_formats: &[],
};
