use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2517232495: FileType = FileType {
    file_format: &FileFormat {
        id: 2_517_232_495,
        source_type: SourceType::Httpd,
        name: "fsc weblaunch",
        extensions: &["fsc"],
        media_types: &["application/vnd.fsc.weblaunch"],
        signatures: &[],
        related_formats: &[],
    },
};
