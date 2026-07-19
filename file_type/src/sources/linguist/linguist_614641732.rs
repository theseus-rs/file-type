use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_614641732: FileType = FileType {
    file_format: &FileFormat {
        id: 614_641_732,
        source_type: SourceType::Linguist,
        name: "Liquidsoap",
        extensions: &["liq"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
