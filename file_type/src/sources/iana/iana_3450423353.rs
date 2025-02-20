use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3450423353: FileType = FileType {
    file_format: &FileFormat {
        id: 3_450_423_353,
        source_type: SourceType::Iana,
        name: "vnd.cluetrust.cartomobile-config-pkg",
        extensions: &[],
        media_types: &["application/vnd.cluetrust.cartomobile-config-pkg"],
        signatures: &[],
        related_formats: &[],
    },
};
