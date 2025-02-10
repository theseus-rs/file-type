use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_10387757: FileType = FileType {
    file_format: &FileFormat {
        id: 10_387_757,
        source_type: SourceType::Wikidata,
        name: "Universal Image Format",
        extensions: &["uif"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
