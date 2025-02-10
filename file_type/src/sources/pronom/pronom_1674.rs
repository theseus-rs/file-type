use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1674: FileType = FileType {
    file_format: &FileFormat {
        id: 1_674,
        source_type: SourceType::Pronom,
        name: "Perl Script",
        extensions: &["pl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
