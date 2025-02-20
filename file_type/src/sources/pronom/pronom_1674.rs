use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
