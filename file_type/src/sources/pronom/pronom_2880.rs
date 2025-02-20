use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_2880: FileType = FileType {
    file_format: &FileFormat {
        id: 2_880,
        source_type: SourceType::Pronom,
        name: "Compressed MusicXML",
        extensions: &["mxl"],
        media_types: &["application/vnd.recordare.musicxml"],
        signatures: &[],
        related_formats: &[],
    },
};
