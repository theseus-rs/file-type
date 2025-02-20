use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_1701: FileType = FileType {
    file_format: &FileFormat {
        id: 1_701,
        source_type: SourceType::Pronom,
        name: "Compressed MusicXML",
        extensions: &["mxl"],
        media_types: &["application/vnd.recordare.musicxml"],
        signatures: &[],
        related_formats: &[],
    },
};
