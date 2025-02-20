use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2383649697: FileType = FileType {
    file_format: &FileFormat {
        id: 2_383_649_697,
        source_type: SourceType::Httpd,
        name: "tcl",
        extensions: &["tcl"],
        media_types: &["application/x-tcl"],
        signatures: &[],
        related_formats: &[],
    },
};
