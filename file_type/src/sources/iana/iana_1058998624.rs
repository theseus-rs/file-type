use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1058998624: FileType = FileType {
    file_format: &FileFormat {
        id: 1_058_998_624,
        source_type: SourceType::Iana,
        name: "vnd.motorola.flexsuite",
        extensions: &[],
        media_types: &["application/vnd.motorola.flexsuite"],
        signatures: &[],
        related_formats: &[],
    },
};
