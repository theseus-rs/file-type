use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136753455: FileType = FileType {
    file_format: &FileFormat {
        id: 136_753_455,
        source_type: SourceType::Wikidata,
        name: "Adobe Tool Preset",
        extensions: &["tpl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
