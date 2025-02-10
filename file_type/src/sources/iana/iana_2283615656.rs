use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2283615656: FileType = FileType {
    file_format: &FileFormat {
        id: 2_283_615_656,
        source_type: SourceType::Iana,
        name: "vnd.nuera.ecelp4800",
        extensions: &[],
        media_types: &["audio/vnd.nuera.ecelp4800"],
        signatures: &[],
        related_formats: &[],
    },
};
