use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1218352227: FileFormat = FileFormat {
    id: 1_218_352_227,
    source_type: SourceType::Httpd,
    name: "realvnc bed",
    extensions: &["bed"],
    media_types: &["application/vnd.realvnc.bed"],
    internal_signatures: &[],
    related_formats: &[],
};
