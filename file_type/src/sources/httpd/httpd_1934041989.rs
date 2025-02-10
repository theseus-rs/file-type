use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1934041989: FileType = FileType {
    file_format: &FileFormat {
        id: 1_934_041_989,
        source_type: SourceType::Httpd,
        name: "ms xpsdocument",
        extensions: &["xps"],
        media_types: &["application/vnd.ms-xpsdocument"],
        signatures: &[],
        related_formats: &[],
    },
};
