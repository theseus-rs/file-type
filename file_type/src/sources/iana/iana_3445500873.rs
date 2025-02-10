use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3445500873: FileType = FileType {
    file_format: &FileFormat {
        id: 3_445_500_873,
        source_type: SourceType::Iana,
        name: "vnd.ms-tnef",
        extensions: &[],
        media_types: &["application/vnd.ms-tnef"],
        signatures: &[],
        related_formats: &[],
    },
};
