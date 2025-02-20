use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2372075112: FileType = FileType {
    file_format: &FileFormat {
        id: 2_372_075_112,
        source_type: SourceType::Iana,
        name: "vc+jwt",
        extensions: &[],
        media_types: &["application/vc+jwt"],
        signatures: &[],
        related_formats: &[],
    },
};
