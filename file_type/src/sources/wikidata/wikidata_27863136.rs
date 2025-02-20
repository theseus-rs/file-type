use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27863136: FileType = FileType {
    file_format: &FileFormat {
        id: 27_863_136,
        source_type: SourceType::Wikidata,
        name: "AutoCAD Drawing, version 2007-2008",
        extensions: &["dwg"],
        media_types: &["image/vnd.dwg"],
        signatures: &[],
        related_formats: &[],
    },
};
