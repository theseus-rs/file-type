use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3513520853: FileType = FileType {
    file_format: &FileFormat {
        id: 3_513_520_853,
        source_type: SourceType::Iana,
        name: "vnd.exchangeable",
        extensions: &[],
        media_types: &["text/vnd.exchangeable"],
        signatures: &[],
        related_formats: &[],
    },
};
