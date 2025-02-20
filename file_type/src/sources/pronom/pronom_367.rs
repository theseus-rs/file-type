use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_367: FileType = FileType {
    file_format: &FileFormat {
        id: 367,
        source_type: SourceType::Pronom,
        name: "Microsoft Outlook Personal Folders",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
