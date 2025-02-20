use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_210859652: FileType = FileType {
    file_format: &FileFormat {
        id: 210_859_652,
        source_type: SourceType::Iana,
        name: "vnd.fuzzysheet",
        extensions: &[],
        media_types: &["application/vnd.fuzzysheet"],
        signatures: &[],
        related_formats: &[],
    },
};
