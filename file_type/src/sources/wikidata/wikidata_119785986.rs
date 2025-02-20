use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_119785986: FileType = FileType {
    file_format: &FileFormat {
        id: 119_785_986,
        source_type: SourceType::Wikidata,
        name: "MasterCook Calendar File",
        extensions: &["mcl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
