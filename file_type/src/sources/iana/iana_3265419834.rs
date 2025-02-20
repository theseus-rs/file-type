use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3265419834: FileType = FileType {
    file_format: &FileFormat {
        id: 3_265_419_834,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.theme+xml",
        extensions: &[],
        media_types: &["application/vnd.openxmlformats-officedocument.theme+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
