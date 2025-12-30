use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136539560: FileType = FileType {
    file_format: &FileFormat {
        id: 136_539_560,
        source_type: SourceType::Wikidata,
        name: "MIX",
        extensions: &["mix"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
