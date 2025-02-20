use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_131: FileType = FileType {
    file_format: &FileFormat {
        id: 131,
        source_type: SourceType::Pronom,
        name: "Microsoft Powerpoint Add-In",
        extensions: &["ppa"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
