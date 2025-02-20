use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
