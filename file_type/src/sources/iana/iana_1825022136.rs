use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1825022136: FileType = FileType {
    file_format: &FileFormat {
        id: 1_825_022_136,
        source_type: SourceType::Iana,
        name: "vnd.informix-visionary (OBSOLETED in favor of application/vnd.visionary)",
        extensions: &[],
        media_types: &["application/vnd.informix-visionary"],
        signatures: &[],
        related_formats: &[],
    },
};
