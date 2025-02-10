use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28600263: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_263,
        source_type: SourceType::Wikidata,
        name: "Ability Database",
        extensions: &["adb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
