use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117035149: FileType = FileType {
    file_format: &FileFormat {
        id: 117_035_149,
        source_type: SourceType::Wikidata,
        name: "TurboCAD for Windows ASCII File",
        extensions: &["tcx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
