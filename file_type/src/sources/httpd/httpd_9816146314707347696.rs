use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_9816146314707347696: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "applixware",
    extensions: &["aw"],
    media_types: &["application/applixware"],
    internal_signatures: &[],
    related_formats: &[],
};
