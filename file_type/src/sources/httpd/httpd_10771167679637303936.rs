use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_10771167679637303936: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "yamaha hv dic",
    extensions: &["hvd"],
    media_types: &["application/vnd.yamaha.hv-dic"],
    internal_signatures: &[],
    related_formats: &[],
};
