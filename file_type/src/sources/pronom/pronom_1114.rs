use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1114: FileType = FileType {
    file_format: &FileFormat {
        id: 1_114,
        source_type: SourceType::Pronom,
        name: "ESRI World File Format",
        extensions: &[
            "tfw", "jgw", "pgw", "bpw", "tifw", "blw", "bilw", "jpgw", "rasterw", "btw", "sdw",
        ],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
