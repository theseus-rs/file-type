use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_84997326: FileType = FileType {
    file_format: &FileFormat {
        id: 84_997_326,
        source_type: SourceType::Wikidata,
        name: "Autodesk Revit File, version 4",
        extensions: &["rfa", "rft", "rte", "rvt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
