use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2077993683: FileType = FileType {
    file_format: &FileFormat {
        id: 2_077_993_683,
        source_type: SourceType::Iana,
        name: "vnd.ibm.electronic-media",
        extensions: &[],
        media_types: &["application/vnd.ibm.electronic-media"],
        signatures: &[],
        related_formats: &[],
    },
};
