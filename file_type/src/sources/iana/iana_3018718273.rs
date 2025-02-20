use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3018718273: FileType = FileType {
    file_format: &FileFormat {
        id: 3_018_718_273,
        source_type: SourceType::Iana,
        name: "n-quads",
        extensions: &[],
        media_types: &["application/n-quads"],
        signatures: &[],
        related_formats: &[],
    },
};
