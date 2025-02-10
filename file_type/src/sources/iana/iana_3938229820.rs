use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3938229820: FileType = FileType {
    file_format: &FileFormat {
        id: 3_938_229_820,
        source_type: SourceType::Iana,
        name: "vnd.sus-calendar",
        extensions: &[],
        media_types: &["application/vnd.sus-calendar"],
        signatures: &[],
        related_formats: &[],
    },
};
