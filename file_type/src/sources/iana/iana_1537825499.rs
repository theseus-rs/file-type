use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1537825499: FileType = FileType {
    file_format: &FileFormat {
        id: 1_537_825_499,
        source_type: SourceType::Iana,
        name: "vnd.zoo.kcl",
        extensions: &[],
        media_types: &["text/vnd.zoo.kcl"],
        signatures: &[],
        related_formats: &[],
    },
};
