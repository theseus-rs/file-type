use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3613076564: FileType = FileType {
    file_format: &FileFormat {
        id: 3_613_076_564,
        source_type: SourceType::Iana,
        name: "vnd.wmap",
        extensions: &[],
        media_types: &["application/vnd.wmap"],
        signatures: &[],
        related_formats: &[],
    },
};
