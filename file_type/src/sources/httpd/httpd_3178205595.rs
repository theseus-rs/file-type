use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3178205595: FileType = FileType {
    file_format: &FileFormat {
        id: 3_178_205_595,
        source_type: SourceType::Httpd,
        name: "iso9660 image",
        extensions: &["iso"],
        media_types: &["application/x-iso9660-image"],
        signatures: &[],
        related_formats: &[],
    },
};
