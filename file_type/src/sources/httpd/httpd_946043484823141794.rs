use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_946043484823141794: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "tmobile livetv",
    extensions: &["tmo"],
    media_types: &["application/vnd.tmobile-livetv"],
    internal_signatures: &[],
    related_formats: &[],
};
