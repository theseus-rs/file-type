use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_721217963: FileType = FileType {
    file_format: &FileFormat {
        id: 721_217_963,
        source_type: SourceType::Iana,
        name: "vnd.neurolanguage.nlu",
        extensions: &[],
        media_types: &["application/vnd.neurolanguage.nlu"],
        signatures: &[],
        related_formats: &[],
    },
};
