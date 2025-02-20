use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2490457406: FileType = FileType {
    file_format: &FileFormat {
        id: 2_490_457_406,
        source_type: SourceType::Httpd,
        name: "font linux psf",
        extensions: &["psf"],
        media_types: &["application/x-font-linux-psf"],
        signatures: &[],
        related_formats: &[],
    },
};
