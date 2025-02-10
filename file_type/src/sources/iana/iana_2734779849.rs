use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2734779849: FileType = FileType {
    file_format: &FileFormat {
        id: 2_734_779_849,
        source_type: SourceType::Iana,
        name: "vnd.gtw",
        extensions: &[],
        media_types: &["model/vnd.gtw"],
        signatures: &[],
        related_formats: &[],
    },
};
