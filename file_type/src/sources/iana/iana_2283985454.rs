use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2283985454: FileType = FileType {
    file_format: &FileFormat {
        id: 2_283_985_454,
        source_type: SourceType::Iana,
        name: "nasdata",
        extensions: &[],
        media_types: &["application/nasdata"],
        signatures: &[],
        related_formats: &[],
    },
};
