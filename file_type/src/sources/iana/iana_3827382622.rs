use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3827382622: FileType = FileType {
    file_format: &FileFormat {
        id: 3_827_382_622,
        source_type: SourceType::Iana,
        name: "mac-binhex40",
        extensions: &[],
        media_types: &["application/mac-binhex40"],
        signatures: &[],
        related_formats: &[],
    },
};
