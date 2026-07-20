use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_6542498: FileType = FileType {
    file_format: &FileFormat {
        id: 6_542_498,
        source_type: SourceType::Wikidata,
        name: "Library Exchange Format",
        extensions: &["lef"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
