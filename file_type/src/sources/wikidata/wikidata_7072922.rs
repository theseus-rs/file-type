use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_7072922: FileType = FileType {
    file_format: &FileFormat {
        id: 7_072_922,
        source_type: SourceType::Wikidata,
        name: "Olympus Raw Format",
        extensions: &["orf"],
        media_types: &["image/x-olympus-orf", "image/x-raw-olympus"],
        signatures: &[],
        related_formats: &[],
    },
};
