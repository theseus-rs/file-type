use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27863143: FileType = FileType {
    file_format: &FileFormat {
        id: 27_863_143,
        source_type: SourceType::Wikidata,
        name: "AutoCAD Drawing, version 2013-2014",
        extensions: &["dwg"],
        media_types: &["image/vnd.dwg"],
        signatures: &[],
        related_formats: &[],
    },
};
