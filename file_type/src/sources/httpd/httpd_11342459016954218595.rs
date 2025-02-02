use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_11342459016954218595: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "syncml dm wbxml",
    extensions: &["bdm"],
    media_types: &["application/vnd.syncml.dm+wbxml"],
    internal_signatures: &[],
    related_formats: &[],
};
