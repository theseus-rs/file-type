use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27863110: FileType = FileType {
    file_format: &FileFormat {
        id: 27_863_110,
        source_type: SourceType::Wikidata,
        name: "AutoCAD Drawing, version 1.3",
        extensions: &["dwg"],
        media_types: &["image/vnd.dwg"],
        signatures: &[],
        related_formats: &[],
    },
};
