use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_8243185071847872480: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "mif",
    extensions: &["mif"],
    media_types: &["application/vnd.mif"],
    internal_signatures: &[],
    related_formats: &[],
};
