use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1685356633: FileType = FileType {
    file_format: &FileFormat {
        id: 1_685_356_633,
        source_type: SourceType::Iana,
        name: "vnd.ms-powerpoint.template.macroEnabled.12",
        extensions: &[],
        media_types: &["application/vnd.ms-powerpoint.template.macroEnabled.12"],
        signatures: &[],
        related_formats: &[],
    },
};
