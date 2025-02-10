use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_694995663: FileType = FileType {
    file_format: &FileFormat {
        id: 694_995_663,
        source_type: SourceType::Httpd,
        name: "cooltalk",
        extensions: &["ice"],
        media_types: &["x-conference/x-cooltalk"],
        signatures: &[],
        related_formats: &[],
    },
};
