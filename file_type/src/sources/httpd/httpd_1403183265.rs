use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1403183265: FileType = FileType {
    file_format: &FileFormat {
        id: 1_403_183_265,
        source_type: SourceType::Httpd,
        name: "americandynamics acc",
        extensions: &["acc"],
        media_types: &["application/vnd.americandynamics.acc"],
        signatures: &[],
        related_formats: &[],
    },
};
