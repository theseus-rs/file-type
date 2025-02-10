use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1727175336: FileType = FileType {
    file_format: &FileFormat {
        id: 1_727_175_336,
        source_type: SourceType::Httpd,
        name: "dts hd",
        extensions: &["dtshd"],
        media_types: &["audio/vnd.dts.hd"],
        signatures: &[],
        related_formats: &[],
    },
};
