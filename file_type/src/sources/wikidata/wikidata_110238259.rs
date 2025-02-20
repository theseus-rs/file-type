use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110238259: FileType = FileType {
    file_format: &FileFormat {
        id: 110_238_259,
        source_type: SourceType::Wikidata,
        name: "Dramatica/StoryView Exchange",
        extensions: &["dsw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
