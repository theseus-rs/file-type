use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_85027567: FileType = FileType {
    file_format: &FileFormat {
        id: 85_027_567,
        source_type: SourceType::Wikidata,
        name: "Autodesk Revit Family File, version 2008",
        extensions: &["rfa", "rft", "rte", "rvt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
