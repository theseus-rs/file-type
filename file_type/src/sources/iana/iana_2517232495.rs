use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2517232495: FileFormat = FileFormat {
    id: 2_517_232_495,
    source_type: SourceType::Iana,
    name: "vnd.fsc.weblaunch",
    extensions: &[],
    media_types: &["application/vnd.fsc.weblaunch"],
    internal_signatures: &[],
    related_formats: &[],
};
