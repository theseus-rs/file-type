use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28345358: FileType = FileType {
    file_format: &FileFormat {
        id: 28_345_358,
        source_type: SourceType::Wikidata,
        name: "Safari bookmarks",
        extensions: &["plist"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
