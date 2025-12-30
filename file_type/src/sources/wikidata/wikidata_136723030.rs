use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136723030: FileType = FileType {
    file_format: &FileFormat {
        id: 136_723_030,
        source_type: SourceType::Wikidata,
        name: "REAPER project file",
        extensions: &["rpp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
