use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3942621849: FileType = FileType {
    file_format: &FileFormat {
        id: 3_942_621_849,
        source_type: SourceType::Httpd,
        name: "mobius txf",
        extensions: &["txf"],
        media_types: &["application/vnd.mobius.txf"],
        signatures: &[],
        related_formats: &[],
    },
};
