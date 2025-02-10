use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_66663925: FileType = FileType {
    file_format: &FileFormat {
        id: 66_663_925,
        source_type: SourceType::Wikidata,
        name: "OS/2 Metafile",
        extensions: &["met"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
