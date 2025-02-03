use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3942621849: FileFormat = FileFormat {
    id: 3_942_621_849,
    source_type: SourceType::Httpd,
    name: "mobius txf",
    extensions: &["txf"],
    media_types: &["application/vnd.mobius.txf"],
    internal_signatures: &[],
    related_formats: &[],
};
