use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3679007507: FileType = FileType {
    file_format: &FileFormat {
        id: 3_679_007_507,
        source_type: SourceType::Httpd,
        name: "conference",
        extensions: &["nsc"],
        media_types: &["application/x-conference"],
        signatures: &[],
        related_formats: &[],
    },
};
