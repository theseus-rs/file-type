use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1435102575: FileFormat = FileFormat {
    id: 1_435_102_575,
    source_type: SourceType::Httpd,
    name: "yamaha hv dic",
    extensions: &["hvd"],
    media_types: &["application/vnd.yamaha.hv-dic"],
    internal_signatures: &[],
    related_formats: &[],
};
