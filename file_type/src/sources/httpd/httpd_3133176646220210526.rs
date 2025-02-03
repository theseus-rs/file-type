use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3133176646220210526: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "hydrostatix sof data",
    extensions: &["sfd-hdstx"],
    media_types: &["application/vnd.hydrostatix.sof-data"],
    internal_signatures: &[],
    related_formats: &[],
};
