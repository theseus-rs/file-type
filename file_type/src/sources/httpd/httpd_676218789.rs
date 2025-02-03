use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_676218789: FileFormat = FileFormat {
    id: 676_218_789,
    source_type: SourceType::Httpd,
    name: "dgc compressed",
    extensions: &["dgc"],
    media_types: &["application/x-dgc-compressed"],
    internal_signatures: &[],
    related_formats: &[],
};
