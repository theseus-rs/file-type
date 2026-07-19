use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1661052: FileType = FileType {
    file_format: &FileFormat {
        id: 1_661_052,
        source_type: SourceType::Wikidata,
        name: "Indeo",
        extensions: &["ivf"],
        media_types: &["video/x-indeo", "video/x-ivf"],
        signatures: &[],
        related_formats: &[],
    },
};
