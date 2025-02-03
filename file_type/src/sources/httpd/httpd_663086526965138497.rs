use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_663086526965138497: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "ms vob",
    extensions: &["vob"],
    media_types: &["video/x-ms-vob"],
    internal_signatures: &[],
    related_formats: &[],
};
