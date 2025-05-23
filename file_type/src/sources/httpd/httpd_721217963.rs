use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_721217963: FileType = FileType {
    file_format: &FileFormat {
        id: 721_217_963,
        source_type: SourceType::Httpd,
        name: "neurolanguage nlu",
        extensions: &["nlu"],
        media_types: &["application/vnd.neurolanguage.nlu"],
        signatures: &[],
        related_formats: &[],
    },
};
