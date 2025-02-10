use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2760154822: FileType = FileType {
    file_format: &FileFormat {
        id: 2_760_154_822,
        source_type: SourceType::Httpd,
        name: "epson esf",
        extensions: &["esf"],
        media_types: &["application/vnd.epson.esf"],
        signatures: &[],
        related_formats: &[],
    },
};
