use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_48906245: FileType = FileType {
    file_format: &FileFormat {
        id: 48_906_245,
        source_type: SourceType::Wikidata,
        name: "Harvard Graphics Vector Graphics",
        extensions: &["cht"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
