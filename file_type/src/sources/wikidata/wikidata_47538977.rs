use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47538977: FileType = FileType {
    file_format: &FileFormat {
        id: 47_538_977,
        source_type: SourceType::Wikidata,
        name: "AutoCAD Template Menu File",
        extensions: &["mnu"],
        media_types: &["application/x-autocad"],
        signatures: &[],
        related_formats: &[],
    },
};
