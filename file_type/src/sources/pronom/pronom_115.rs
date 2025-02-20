use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_115: FileType = FileType {
    file_format: &FileFormat {
        id: 115,
        source_type: SourceType::Pronom,
        name: "Microsoft Outlook Address Book",
        extensions: &["olk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
