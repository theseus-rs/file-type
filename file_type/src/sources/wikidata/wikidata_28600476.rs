use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28600476: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_476,
        source_type: SourceType::Wikidata,
        name: "DOS device driver",
        extensions: &["dos", "sys"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
