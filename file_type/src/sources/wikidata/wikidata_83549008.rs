use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_83549008: FileType = FileType {
    file_format: &FileFormat {
        id: 83_549_008,
        source_type: SourceType::Wikidata,
        name: "Nearly Raw Raster Data, version 5",
        extensions: &["nrrd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
