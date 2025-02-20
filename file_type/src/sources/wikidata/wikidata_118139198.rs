use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_118139198: FileType = FileType {
    file_format: &FileFormat {
        id: 118_139_198,
        source_type: SourceType::Wikidata,
        name: "DOS Caledar File",
        extensions: &["cal"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
