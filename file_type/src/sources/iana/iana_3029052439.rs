use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3029052439: FileType = FileType {
    file_format: &FileFormat {
        id: 3_029_052_439,
        source_type: SourceType::Iana,
        name: "eac3",
        extensions: &[],
        media_types: &["audio/eac3"],
        signatures: &[],
        related_formats: &[],
    },
};
