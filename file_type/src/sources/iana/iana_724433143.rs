use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_724433143: FileType = FileType {
    file_format: &FileFormat {
        id: 724_433_143,
        source_type: SourceType::Iana,
        name: "ohttp-keys",
        extensions: &[],
        media_types: &["application/ohttp-keys"],
        signatures: &[],
        related_formats: &[],
    },
};
