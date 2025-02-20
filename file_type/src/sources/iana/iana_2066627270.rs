use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2066627270: FileType = FileType {
    file_format: &FileFormat {
        id: 2_066_627_270,
        source_type: SourceType::Iana,
        name: "vnd.mif",
        extensions: &[],
        media_types: &["application/vnd.mif"],
        signatures: &[],
        related_formats: &[],
    },
};
