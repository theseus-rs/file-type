use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_17406533731142436187: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "fsc weblaunch",
    extensions: &["fsc"],
    media_types: &["application/vnd.fsc.weblaunch"],
    internal_signatures: &[],
    related_formats: &[],
};
