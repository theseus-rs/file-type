use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27823195: FileType = FileType {
    file_format: &FileFormat {
        id: 27_823_195,
        source_type: SourceType::Wikidata,
        name: "Binary Terrain External Projection file",
        extensions: &["prj"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
