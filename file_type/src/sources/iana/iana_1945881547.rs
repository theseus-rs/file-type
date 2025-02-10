use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1945881547: FileType = FileType {
    file_format: &FileFormat {
        id: 1_945_881_547,
        source_type: SourceType::Iana,
        name: "vnd.oma.lwm2m+tlv",
        extensions: &[],
        media_types: &["application/vnd.oma.lwm2m+tlv"],
        signatures: &[],
        related_formats: &[],
    },
};
