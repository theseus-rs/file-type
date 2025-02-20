use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3305095848: FileType = FileType {
    file_format: &FileFormat {
        id: 3_305_095_848,
        source_type: SourceType::Iana,
        name: "passport",
        extensions: &[],
        media_types: &["application/passport"],
        signatures: &[],
        related_formats: &[],
    },
};
