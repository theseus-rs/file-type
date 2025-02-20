use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1458445620: FileType = FileType {
    file_format: &FileFormat {
        id: 1_458_445_620,
        source_type: SourceType::Iana,
        name: "step-xml+zip",
        extensions: &[],
        media_types: &["model/step-xml+zip"],
        signatures: &[],
        related_formats: &[],
    },
};
