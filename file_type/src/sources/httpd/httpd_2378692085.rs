use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2378692085: FileType = FileType {
    file_format: &FileFormat {
        id: 2_378_692_085,
        source_type: SourceType::Httpd,
        name: "mspublisher",
        extensions: &["pub"],
        media_types: &["application/x-mspublisher"],
        signatures: &[],
        related_formats: &[],
    },
};
