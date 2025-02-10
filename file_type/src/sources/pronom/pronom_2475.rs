use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2475: FileType = FileType {
    file_format: &FileFormat {
        id: 2_475,
        source_type: SourceType::Pronom,
        name: "Crystal Reports File",
        extensions: &["rpt"],
        media_types: &["application/x-rpt"],
        signatures: &[],
        related_formats: &[],
    },
};
