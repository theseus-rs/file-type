use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2118978808: FileType = FileType {
    file_format: &FileFormat {
        id: 2_118_978_808,
        source_type: SourceType::Httpd,
        name: "syncml dm xml",
        extensions: &["xdm"],
        media_types: &["application/vnd.syncml.dm+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
