use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_11770528808789065522: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "glulx",
    extensions: &["ulx"],
    media_types: &["application/x-glulx"],
    internal_signatures: &[],
    related_formats: &[],
};
