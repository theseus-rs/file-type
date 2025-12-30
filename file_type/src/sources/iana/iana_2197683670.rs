use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2197683670: FileType = FileType {
    file_format: &FileFormat {
        id: 2_197_683_670,
        source_type: SourceType::Iana,
        name: "3gpp-media-delivery-metrics-report+json",
        extensions: &[],
        media_types: &["application/3gpp-media-delivery-metrics-report+json"],
        signatures: &[],
        related_formats: &[],
    },
};
