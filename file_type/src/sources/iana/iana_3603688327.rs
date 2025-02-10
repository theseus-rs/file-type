use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3603688327: FileType = FileType {
    file_format: &FileFormat {
        id: 3_603_688_327,
        source_type: SourceType::Iana,
        name: "vnd.evolv.ecig.profile",
        extensions: &[],
        media_types: &["application/vnd.evolv.ecig.profile"],
        signatures: &[],
        related_formats: &[],
    },
};
