use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27995538: FileType = FileType {
    file_format: &FileFormat {
        id: 27_995_538,
        source_type: SourceType::Wikidata,
        name: "Borderlands save file",
        extensions: &["sav"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
