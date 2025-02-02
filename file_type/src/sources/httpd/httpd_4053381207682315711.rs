use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4053381207682315711: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "kde kword",
    extensions: &["kwd", "kwt"],
    media_types: &["application/vnd.kde.kword"],
    internal_signatures: &[],
    related_formats: &[],
};
