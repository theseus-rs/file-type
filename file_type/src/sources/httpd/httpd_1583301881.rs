use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1583301881: FileFormat = FileFormat {
    id: 1_583_301_881,
    source_type: SourceType::Httpd,
    name: "intu qfx",
    extensions: &["qfx"],
    media_types: &["application/vnd.intu.qfx"],
    internal_signatures: &[],
    related_formats: &[],
};
