use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3641071795: FileFormat = FileFormat {
    id: 3_641_071_795,
    source_type: SourceType::Httpd,
    name: "ibm secure container",
    extensions: &["sc"],
    media_types: &["application/vnd.ibm.secure-container"],
    signatures: &[],
    related_formats: &[],
};
