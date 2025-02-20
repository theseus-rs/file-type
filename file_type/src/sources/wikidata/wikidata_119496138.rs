use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_119496138: FileType = FileType {
    file_format: &FileFormat {
        id: 119_496_138,
        source_type: SourceType::Wikidata,
        name: "WinFax format",
        extensions: &["wfx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
