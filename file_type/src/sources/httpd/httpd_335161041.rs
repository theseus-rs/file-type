use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_335161041: FileType = FileType {
    file_format: &FileFormat {
        id: 335_161_041,
        source_type: SourceType::Httpd,
        name: "hydrostatix sof data",
        extensions: &["sfd-hdstx"],
        media_types: &["application/vnd.hydrostatix.sof-data"],
        signatures: &[],
        related_formats: &[],
    },
};
