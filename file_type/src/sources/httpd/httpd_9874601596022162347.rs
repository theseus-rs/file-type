use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_9874601596022162347: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "fvt",
    extensions: &["fvt"],
    media_types: &["video/vnd.fvt"],
    internal_signatures: &[],
    related_formats: &[],
};
