use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1024029615: FileType = FileType {
    file_format: &FileFormat {
        id: 1_024_029_615,
        source_type: SourceType::Iana,
        name: "vnd.commerce-battelle",
        extensions: &[],
        media_types: &["application/vnd.commerce-battelle"],
        signatures: &[],
        related_formats: &[],
    },
};
