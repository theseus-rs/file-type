use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_119406880: FileType = FileType {
    file_format: &FileFormat {
        id: 119_406_880,
        source_type: SourceType::Wikidata,
        name: "ACT! Data File",
        extensions: &["adf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
