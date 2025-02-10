use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110984365: FileType = FileType {
    file_format: &FileFormat {
        id: 110_984_365,
        source_type: SourceType::Wikidata,
        name: "Corel VideoStudio HTML5 Project File",
        extensions: &["vsh"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
