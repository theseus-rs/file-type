use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_85013182: FileType = FileType {
    file_format: &FileFormat {
        id: 85_013_182,
        source_type: SourceType::Wikidata,
        name: "Autodesk Revit Project File, version 2008",
        extensions: &["rfa", "rft", "rte", "rvt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
