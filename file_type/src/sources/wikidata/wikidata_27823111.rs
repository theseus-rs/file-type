use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27823111: FileType = FileType {
    file_format: &FileFormat {
        id: 27_823_111,
        source_type: SourceType::Wikidata,
        name: "Bathymetry Attributed Grid",
        extensions: &["bag"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
