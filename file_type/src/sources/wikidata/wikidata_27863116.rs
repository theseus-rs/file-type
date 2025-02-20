use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27863116: FileType = FileType {
    file_format: &FileFormat {
        id: 27_863_116,
        source_type: SourceType::Wikidata,
        name: "AutoCAD Drawing, version 2.1",
        extensions: &["dwg"],
        media_types: &["image/vnd.dwg"],
        signatures: &[],
        related_formats: &[],
    },
};
