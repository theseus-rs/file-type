use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_5868147151211495659: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "kde kchart",
    extensions: &["chrt"],
    media_types: &["application/vnd.kde.kchart"],
    internal_signatures: &[],
    related_formats: &[],
};
