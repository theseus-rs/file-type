use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4115313787: FileType = FileType {
    file_format: &FileFormat {
        id: 4_115_313_787,
        source_type: SourceType::Iana,
        name: "vnd.preminet",
        extensions: &[],
        media_types: &["application/vnd.preminet"],
        signatures: &[],
        related_formats: &[],
    },
};
