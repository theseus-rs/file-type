use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3510790454: FileType = FileType {
    file_format: &FileFormat {
        id: 3_510_790_454,
        source_type: SourceType::Iana,
        name: "vnd.vidsoft.vidconference",
        extensions: &[],
        media_types: &["application/vnd.vidsoft.vidconference"],
        signatures: &[],
        related_formats: &[],
    },
};
