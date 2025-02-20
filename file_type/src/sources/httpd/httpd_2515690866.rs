use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2515690866: FileType = FileType {
    file_format: &FileFormat {
        id: 2_515_690_866,
        source_type: SourceType::Httpd,
        name: "olpc sugar",
        extensions: &["xo"],
        media_types: &["application/vnd.olpc-sugar"],
        signatures: &[],
        related_formats: &[],
    },
};
