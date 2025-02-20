use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2581876382: FileType = FileType {
    file_format: &FileFormat {
        id: 2_581_876_382,
        source_type: SourceType::Iana,
        name: "cms",
        extensions: &[],
        media_types: &["application/cms"],
        signatures: &[],
        related_formats: &[],
    },
};
