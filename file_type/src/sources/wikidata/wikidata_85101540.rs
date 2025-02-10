use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_85101540: FileType = FileType {
    file_format: &FileFormat {
        id: 85_101_540,
        source_type: SourceType::Wikidata,
        name: "Autodesk Revit Project File 2019",
        extensions: &["rfa", "rft", "rte", "rvt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
