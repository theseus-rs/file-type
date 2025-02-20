use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_119785939: FileType = FileType {
    file_format: &FileFormat {
        id: 119_785_939,
        source_type: SourceType::Wikidata,
        name: "MasterCook Search File",
        extensions: &["src"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
