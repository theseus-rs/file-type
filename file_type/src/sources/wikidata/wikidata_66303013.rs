use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_66303013: FileType = FileType {
    file_format: &FileFormat {
        id: 66_303_013,
        source_type: SourceType::Wikidata,
        name: "Lotus 1-2-3 Educational Version Worksheet file",
        extensions: &["wke"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
