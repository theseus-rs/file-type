use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_16035530569936599317: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "hdf",
    extensions: &["hdf"],
    media_types: &["application/x-hdf"],
    internal_signatures: &[],
    related_formats: &[],
};
