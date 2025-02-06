use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_214394448: FileFormat = FileFormat {
    id: 214_394_448,
    source_type: SourceType::Httpd,
    name: "wap wmlscript",
    extensions: &["wmls"],
    media_types: &["text/vnd.wap.wmlscript"],
    signatures: &[],
    related_formats: &[],
};
