use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_85104101: FileType = FileType {
    file_format: &FileFormat {
        id: 85_104_101,
        source_type: SourceType::Wikidata,
        name: "Autodesk Revit Family File, version 2019",
        extensions: &["rfa", "rft", "rte", "rvt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
