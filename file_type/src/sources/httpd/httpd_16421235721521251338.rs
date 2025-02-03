use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_16421235721521251338: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "xproc xml",
    extensions: &["xpl"],
    media_types: &["application/xproc+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
