use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2517232495: FileFormat = FileFormat {
    id: 2_517_232_495,
    source_type: SourceType::Httpd,
    name: "fsc weblaunch",
    extensions: &["fsc"],
    media_types: &["application/vnd.fsc.weblaunch"],
    internal_signatures: &[],
    related_formats: &[],
};
