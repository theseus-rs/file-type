use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_83548831: FileType = FileType {
    file_format: &FileFormat {
        id: 83_548_831,
        source_type: SourceType::Wikidata,
        name: "Nearly Raw Raster Data, version 2",
        extensions: &["nrrd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
