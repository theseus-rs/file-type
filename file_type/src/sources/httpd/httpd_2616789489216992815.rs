use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2616789489216992815: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "yamaha smaf phrase",
    extensions: &["spf"],
    media_types: &["application/vnd.yamaha.smaf-phrase"],
    internal_signatures: &[],
    related_formats: &[],
};
