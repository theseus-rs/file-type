use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111363669: FileType = FileType {
    file_format: &FileFormat {
        id: 111_363_669,
        source_type: SourceType::Wikidata,
        name: "Wusikstation V3 sound file",
        extensions: &["wusiksnd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
