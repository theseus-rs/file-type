use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_513487635: FileType = FileType {
    file_format: &FileFormat {
        id: 513_487_635,
        source_type: SourceType::Httpd,
        name: "n3",
        extensions: &["n3"],
        media_types: &["text/n3"],
        signatures: &[],
        related_formats: &[],
    },
};
