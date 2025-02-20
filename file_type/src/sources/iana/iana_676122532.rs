use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_676122532: FileType = FileType {
    file_format: &FileFormat {
        id: 676_122_532,
        source_type: SourceType::Iana,
        name: "jaii",
        extensions: &[],
        media_types: &["image/jaii"],
        signatures: &[],
        related_formats: &[],
    },
};
