use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2553: FileType = FileType {
    file_format: &FileFormat {
        id: 2_553,
        source_type: SourceType::Pronom,
        name: "Time Stamp Token",
        extensions: &["tst"],
        media_types: &["application/vnd.etsi.timestamp-token"],
        signatures: &[],
        related_formats: &[],
    },
};
