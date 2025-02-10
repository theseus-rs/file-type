use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3387561077: FileType = FileType {
    file_format: &FileFormat {
        id: 3_387_561_077,
        source_type: SourceType::Iana,
        name: "vnd.wqd",
        extensions: &[],
        media_types: &["application/vnd.wqd"],
        signatures: &[],
        related_formats: &[],
    },
};
