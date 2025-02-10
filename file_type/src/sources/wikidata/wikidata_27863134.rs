use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27863134: FileType = FileType {
    file_format: &FileFormat {
        id: 27_863_134,
        source_type: SourceType::Wikidata,
        name: "AutoCAD Drawing, version 2004-2005",
        extensions: &["dwg"],
        media_types: &["image/vnd.dwg"],
        signatures: &[],
        related_formats: &[],
    },
};
