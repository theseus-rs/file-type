use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28919125: FileType = FileType {
    file_format: &FileFormat {
        id: 28_919_125,
        source_type: SourceType::Wikidata,
        name: "Final Cut Pro X project",
        extensions: &["fcpx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
