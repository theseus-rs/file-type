use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1292606152: FileType = FileType {
    file_format: &FileFormat {
        id: 1_292_606_152,
        source_type: SourceType::Httpd,
        name: "texinfo",
        extensions: &["texinfo", "texi"],
        media_types: &["application/x-texinfo"],
        signatures: &[],
        related_formats: &[],
    },
};
