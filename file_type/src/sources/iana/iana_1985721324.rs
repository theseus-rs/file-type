use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1985721324: FileType = FileType {
    file_format: &FileFormat {
        id: 1_985_721_324,
        source_type: SourceType::Iana,
        name: "vnd.sld",
        extensions: &[],
        media_types: &["image/vnd.sld"],
        signatures: &[],
        related_formats: &[],
    },
};
