use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2472612683: FileType = FileType {
    file_format: &FileFormat {
        id: 2_472_612_683,
        source_type: SourceType::Httpd,
        name: "accpac simply aso",
        extensions: &["aso"],
        media_types: &["application/vnd.accpac.simply.aso"],
        signatures: &[],
        related_formats: &[],
    },
};
