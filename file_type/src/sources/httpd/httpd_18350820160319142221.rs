use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_18350820160319142221: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "syncml dm xml",
    extensions: &["xdm"],
    media_types: &["application/vnd.syncml.dm+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
