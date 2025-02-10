use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27863105: FileType = FileType {
    file_format: &FileFormat {
        id: 27_863_105,
        source_type: SourceType::Wikidata,
        name: "AutoCAD Drawing, version 1.0",
        extensions: &["dwg"],
        media_types: &["image/vnd.dwg"],
        signatures: &[],
        related_formats: &[],
    },
};
