use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_767169629: FileType = FileType {
    file_format: &FileFormat {
        id: 767_169_629,
        source_type: SourceType::Linguist,
        name: "Spline Font Database",
        extensions: &["sfd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
