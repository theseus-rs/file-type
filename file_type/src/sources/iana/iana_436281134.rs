use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_436281134: FileType = FileType {
    file_format: &FileFormat {
        id: 436_281_134,
        source_type: SourceType::Iana,
        name: "vnd.curl",
        extensions: &[],
        media_types: &["application/vnd.curl"],
        signatures: &[],
        related_formats: &[],
    },
};
