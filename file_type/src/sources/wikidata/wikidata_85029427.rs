use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_85029427: FileType = FileType {
    file_format: &FileFormat {
        id: 85_029_427,
        source_type: SourceType::Wikidata,
        name: "Autodesk Revit Family File, version 2010",
        extensions: &["rfa", "rft", "rte", "rvt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
