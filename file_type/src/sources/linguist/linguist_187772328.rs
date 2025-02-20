use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_187772328: FileType = FileType {
    file_format: &FileFormat {
        id: 187_772_328,
        source_type: SourceType::Linguist,
        name: "Altium Designer",
        extensions: &["OutJob", "PcbDoc", "PrjPCB", "SchDoc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
