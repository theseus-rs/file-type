use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2128091057912424249: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "ibm secure container",
    extensions: &["sc"],
    media_types: &["application/vnd.ibm.secure-container"],
    internal_signatures: &[],
    related_formats: &[],
};
