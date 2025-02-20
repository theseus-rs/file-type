use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_2841: FileType = FileType {
    file_format: &FileFormat {
        id: 2_841,
        source_type: SourceType::Pronom,
        name: "Finale Notation File",
        extensions: &["musx"],
        media_types: &["application/vnd.makemusic.notation"],
        signatures: &[],
        related_formats: &[],
    },
};
