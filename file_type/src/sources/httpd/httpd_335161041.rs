use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_335161041: FileFormat = FileFormat {
    id: 335_161_041,
    source_type: SourceType::Httpd,
    name: "hydrostatix sof data",
    extensions: &["sfd-hdstx"],
    media_types: &["application/vnd.hydrostatix.sof-data"],
    internal_signatures: &[],
    related_formats: &[],
};
