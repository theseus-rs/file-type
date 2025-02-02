use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_9903176183460908302: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "oasis opendocument chart template",
    extensions: &["otc"],
    media_types: &["application/vnd.oasis.opendocument.chart-template"],
    internal_signatures: &[],
    related_formats: &[],
};
