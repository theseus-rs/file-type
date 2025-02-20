use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27863111: FileType = FileType {
    file_format: &FileFormat {
        id: 27_863_111,
        source_type: SourceType::Wikidata,
        name: "AutoCAD Drawing, version 1.4",
        extensions: &["dwg"],
        media_types: &["image/vnd.dwg"],
        signatures: &[],
        related_formats: &[],
    },
};
