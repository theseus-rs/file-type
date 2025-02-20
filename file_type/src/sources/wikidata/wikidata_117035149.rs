use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
