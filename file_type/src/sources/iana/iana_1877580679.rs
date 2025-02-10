use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1877580679: FileType = FileType {
    file_format: &FileFormat {
        id: 1_877_580_679,
        source_type: SourceType::Iana,
        name: "GSM",
        extensions: &[],
        media_types: &["audio/GSM"],
        signatures: &[],
        related_formats: &[],
    },
};
