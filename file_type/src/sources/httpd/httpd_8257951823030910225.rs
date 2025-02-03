use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_8257951823030910225: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "nitf",
    extensions: &["ntf", "nitf"],
    media_types: &["application/vnd.nitf"],
    internal_signatures: &[],
    related_formats: &[],
};
