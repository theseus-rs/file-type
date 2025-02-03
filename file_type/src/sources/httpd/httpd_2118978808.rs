use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2118978808: FileFormat = FileFormat {
    id: 2_118_978_808,
    source_type: SourceType::Httpd,
    name: "syncml dm xml",
    extensions: &["xdm"],
    media_types: &["application/vnd.syncml.dm+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
