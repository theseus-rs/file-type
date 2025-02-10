use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_366: FileType = FileType {
    file_format: &FileFormat {
        id: 366,
        source_type: SourceType::Pronom,
        name: "Microsoft Outlook Personal Folders",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
