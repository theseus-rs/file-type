use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3450423353: FileType = FileType {
    file_format: &FileFormat {
        id: 3_450_423_353,
        source_type: SourceType::Httpd,
        name: "cluetrust cartomobile config pkg",
        extensions: &["c11amz"],
        media_types: &["application/vnd.cluetrust.cartomobile-config-pkg"],
        signatures: &[],
        related_formats: &[],
    },
};
