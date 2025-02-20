use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_2180: FileType = FileType {
    file_format: &FileFormat {
        id: 2_180,
        source_type: SourceType::Pronom,
        name: "Microsoft MapPoint Document",
        extensions: &["ptm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
