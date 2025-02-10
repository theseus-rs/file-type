use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2241674470: FileType = FileType {
    file_format: &FileFormat {
        id: 2_241_674_470,
        source_type: SourceType::Iana,
        name: "vnd.motorola.flexsuite.gotap",
        extensions: &[],
        media_types: &["application/vnd.motorola.flexsuite.gotap"],
        signatures: &[],
        related_formats: &[],
    },
};
