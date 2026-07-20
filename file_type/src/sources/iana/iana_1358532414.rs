use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1358532414: FileType = FileType {
    file_format: &FileFormat {
        id: 1_358_532_414,
        source_type: SourceType::Iana,
        name: "cloudevents-batch+json",
        extensions: &[],
        media_types: &["application/cloudevents-batch+json"],
        signatures: &[],
        related_formats: &[],
    },
};
