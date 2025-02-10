use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2205828326: FileType = FileType {
    file_format: &FileFormat {
        id: 2_205_828_326,
        source_type: SourceType::Iana,
        name: "vnd.jsk.isdn-ngn",
        extensions: &[],
        media_types: &["application/vnd.jsk.isdn-ngn"],
        signatures: &[],
        related_formats: &[],
    },
};
