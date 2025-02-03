use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_5622317916550356859: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "mscardfile",
    extensions: &["crd"],
    media_types: &["application/x-mscardfile"],
    internal_signatures: &[],
    related_formats: &[],
};
