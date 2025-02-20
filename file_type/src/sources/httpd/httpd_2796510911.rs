use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2796510911: FileType = FileType {
    file_format: &FileFormat {
        id: 2_796_510_911,
        source_type: SourceType::Httpd,
        name: "oasis opendocument image",
        extensions: &["odi"],
        media_types: &["application/vnd.oasis.opendocument.image"],
        signatures: &[],
        related_formats: &[],
    },
};
