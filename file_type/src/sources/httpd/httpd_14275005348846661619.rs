use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_14275005348846661619: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "fortran",
    extensions: &["f", "for", "f77", "f90"],
    media_types: &["text/x-fortran"],
    internal_signatures: &[],
    related_formats: &[],
};
