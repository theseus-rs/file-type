use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2165814283: FileType = FileType {
    file_format: &FileFormat {
        id: 2_165_814_283,
        source_type: SourceType::Httpd,
        name: "novadigm ext",
        extensions: &["ext"],
        media_types: &["application/vnd.novadigm.ext"],
        signatures: &[],
        related_formats: &[],
    },
};
