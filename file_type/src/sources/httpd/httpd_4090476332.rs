use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4090476332: FileFormat = FileFormat {
    id: 4_090_476_332,
    source_type: SourceType::Httpd,
    name: "oasis opendocument database",
    extensions: &["odb"],
    media_types: &["application/vnd.oasis.opendocument.database"],
    internal_signatures: &[],
    related_formats: &[],
};
