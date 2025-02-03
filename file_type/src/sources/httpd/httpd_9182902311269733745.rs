use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_9182902311269733745: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "spotfire sfs",
    extensions: &["sfs"],
    media_types: &["application/vnd.spotfire.sfs"],
    internal_signatures: &[],
    related_formats: &[],
};
