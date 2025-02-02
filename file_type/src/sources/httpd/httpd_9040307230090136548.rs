use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_9040307230090136548: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "realvnc bed",
    extensions: &["bed"],
    media_types: &["application/vnd.realvnc.bed"],
    internal_signatures: &[],
    related_formats: &[],
};
