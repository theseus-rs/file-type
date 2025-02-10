use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2559689858: FileType = FileType {
    file_format: &FileFormat {
        id: 2_559_689_858,
        source_type: SourceType::Httpd,
        name: "fujitsu oasysgp",
        extensions: &["fg5"],
        media_types: &["application/vnd.fujitsu.oasysgp"],
        signatures: &[],
        related_formats: &[],
    },
};
